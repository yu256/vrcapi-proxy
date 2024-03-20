use super::error::WSError;
use crate::fetcher::{request, ResponseExt as _};
use crate::global::{AUTHORIZATION, FRIENDS, MYSELF};
use crate::internal::user_info::fetch_user_info;
use crate::user::{Status, User};
use crate::websocket::server::STREAM_SENDERS;
use crate::{
    global::{APP_NAME, UA},
    websocket::structs::{LocationEventContent, StreamBody, UserIdContent},
};
use futures::StreamExt as _;
use hyper::Method;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{connect_async, tungstenite};
use WSError::*;

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
                    let content = serde_json::from_str::<LocationEventContent>(&body.content)?;
                    let mut user: User = content.into();
                    user.unsanitize();

                    let friends = &mut FRIENDS.write().await;

                    if let Some(index) = friends.offline.iter().position(|x| x.id == user.id) {
                        friends.offline.remove(index);
                    }

                    if let Some(friend) = friends
                        .online
                        .iter_mut()
                        .find(|friend| friend.id == user.id)
                    {
                        *friend = user;
                    } else {
                        friends.online.push(user);
                    }

                    friends.online.sort();
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

                    if new_friend.location != "offline" {
                        if let Status::AskMe | Status::Busy = new_friend.status {
                            if fetch_user_info(&AUTHORIZATION.1.read().await)
                                .await?
                                .activeFriends
                                .contains(&new_friend.id)
                            {
                                let locked = &mut FRIENDS.write().await;
                                locked.online.push(new_friend);
                                locked.online.sort();
                            } else {
                                let locked = &mut FRIENDS.write().await;
                                locked.web.push(new_friend);
                                locked.web.sort();
                            }
                        } else {
                            let locked = &mut FRIENDS.write().await;
                            locked.online.push(new_friend);
                            locked.online.sort();
                        }
                    } else {
                        let locked = &mut FRIENDS.write().await;
                        locked.offline.push(new_friend);
                        locked.offline.sort();
                    }
                }

                t @ ("friend-offline" | "friend-delete" | "friend-active") => {
                    let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                    let friends = &mut FRIENDS.write().await;

                    macro_rules! move_friend {
                        ([$($from:ident),*], $to:ident) => {
                            'block: {
                                $(
                                    if let Some(index) = friends.$from.iter().position(|x| x.id == id) {
                                        let friend = friends.$from.remove(index);
                                        friends.$to.push(friend);
                                        friends.$to.sort();
                                        break 'block;
                                    }
                                )*
                            }
                        };
                    }

                    macro_rules! remove_friend {
                        ([$($from:ident),*]) => {
                            'block: {
                                $(
                                    if let Some(index) = friends.$from.iter().position(|x| x.id == id) {
                                        friends.$from.remove(index);
                                        break 'block;
                                    }
                                )*
                            }
                        };
                    }

                    match t {
                        "friend-offline" => move_friend!([online, web], offline),
                        "friend-delete" => remove_friend!([online, web, offline]),
                        "friend-active" => move_friend!([online, offline], web),
                        _ => unreachable!(),
                    }
                }

                "user-location" => {
                    let user = serde_json::from_str::<LocationEventContent>(&body.content)?.into();
                    MYSELF.insert(user).await;
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
