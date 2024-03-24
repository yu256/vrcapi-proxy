use super::error::WSError;
use crate::fetcher::{request, ResponseExt as _};
use crate::global::{AUTHORIZATION, USERS};
use crate::internal::user_info::fetch_user_info;
use crate::user::{Status, User};
use crate::websocket::server::STREAM_SENDERS;
use crate::{
    global::{APP_NAME, UA},
    websocket::structs::{FriendActive, FriendLocation, StreamBody, UserIdContent},
};
use futures::StreamExt as _;
use hyper::Method;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{connect_async, tungstenite};
use WSError::*;

macro_rules! remove_friend {
    ($friends:expr, $id:expr, [$($from:ident),*]) => {
        'block: {
            $(
                if $friends.$from.find_remove(|x| x.id == $id).is_some() {
                    break 'block;
                }
            )*
        }
    };
}

pub(super) async fn stream() -> WSError {
    // Safety: トークンがあっているなら失敗するはずがない 不正であればこの関数に到達しない
    let mut req = unsafe {
        format!(
            "wss://pipeline.vrchat.cloud/?{}",
            &*AUTHORIZATION.1.read().await
        )
        .into_client_request()
        .unwrap_unchecked()
    };

    req.headers_mut()
        .insert(UA, HeaderValue::from_static(APP_NAME));

    let (mut stream, _) = match connect_async(req).await {
        Ok(ok) => ok,
        Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
        Err(e) => return Unknown(e.to_string()),
    };

    println!("Connected to the websocket.");

    while let Some(message) = stream.next().await {
        let message = match message {
            Ok(tungstenite::Message::Text(message)) if message.starts_with(r#"{"err"#) => {
                return if message.contains("authToken") {
                    Token
                } else {
                    Unknown(message)
                };
            }
            Ok(tungstenite::Message::Text(message)) => message,
            Ok(tungstenite::Message::Close(_)) | Err(tungstenite::error::Error::Protocol(_)) => {
                return Disconnected;
            }
            Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
            Err(e) => return Unknown(e.to_string()),
            _ => continue,
        };

        tokio::spawn(async move {
            let body = serde_json::from_str::<StreamBody>(&message)?;

            match body.r#type.as_str() {
                "friend-online" | "friend-location" => {
                    let (mut user, _) =
                        serde_json::from_str::<FriendLocation>(&body.content)?.normalize();
                    user.unsanitize();

                    let users = &mut USERS.write().await;

                    if let Some(friend) =
                        users.online.iter_mut().find(|friend| friend.id == user.id)
                    {
                        let need_to_be_sorted = friend.status != user.status;
                        *friend = user;
                        if need_to_be_sorted {
                            users.online.sort();
                        }
                    } else {
                        remove_friend!(users, user.id, [offline, web]);
                        users.online.push_and_sort(user);
                    }
                }

                "friend-active" => {
                    let user = serde_json::from_str::<FriendActive>(&body.content)?.user;
                    let locked = &mut USERS.write().await;
                    remove_friend!(locked, user.id, [offline, online, web]);
                    locked.web.push_and_sort(user);
                }

                "friend-add" => {
                    let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                    let mut new_friend = request(
                        Method::GET,
                        &format!("https://api.vrchat.cloud/api/1/users/{id}"),
                        &AUTHORIZATION.1.read().await,
                    )
                    .await?
                    .json::<User>()
                    .await?;

                    new_friend.unsanitize();

                    let write = USERS.write();

                    if new_friend.location.as_ref().is_some_and(|l| l != "offline") {
                        if let Status::AskMe | Status::Busy = new_friend.status {
                            if fetch_user_info(&AUTHORIZATION.1.read().await)
                                .await?
                                .activeFriends
                                .contains(&new_friend.id)
                            {
                                write.await.web.push_and_sort(new_friend);
                            } else {
                                write.await.online.push_and_sort(new_friend);
                            }
                        } else {
                            write.await.online.push_and_sort(new_friend);
                        }
                    } else {
                        write.await.offline.push_and_sort(new_friend);
                    }
                }

                "friend-delete" => {
                    let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                    let locked = &mut USERS.write().await;
                    remove_friend!(locked, id, [offline, web, online]);
                }

                "friend-offline" => {
                    let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                    let locked = &mut USERS.write().await;

                    if let Some(mut friend) = locked
                        .online
                        .find_remove(|x| x.id == id)
                        .or_else(|| locked.web.find_remove(|x| x.id == id))
                    {
                        friend.status = Default::default();
                        friend.location = Default::default();
                        friend.travelingToLocation = Default::default();
                        locked.offline.push(friend);
                    }
                }

                "user-location" => {
                    let myself = serde_json::from_str::<FriendLocation>(&body.content)?
                        .normalize()
                        .0;
                    USERS.write().await.myself = Some(myself);
                }

                _ => {}
            }

            let message = Arc::new(message);

            for (_, sender) in STREAM_SENDERS.lock().await.iter_mut() {
                sender.send(Arc::clone(&message)).await?;
            }

            Ok::<(), anyhow::Error>(())
        });
    }

    Disconnected
}

trait VecExt<T> {
    fn find_remove<F>(&mut self, fun: F) -> Option<T>
    where
        F: Fn(&T) -> bool;
    fn push_and_sort(&mut self, item: T)
    where
        T: Ord;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove<F>(&mut self, fun: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().position(fun).map(|i| self.remove(i))
    }
    fn push_and_sort(&mut self, item: T)
    where
        T: Ord,
    {
        self.push(item);
        self.sort();
    }
}
