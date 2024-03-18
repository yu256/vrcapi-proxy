use super::error::WSError;
use crate::fetcher::{request, ResponseExt as _};
use crate::global::{AUTHORIZATION, FRIENDS, MYSELF};
use crate::user::{Status, User, VecUserExt as _};
use crate::websocket::server::STREAM_SENDERS;
use crate::{
    global::{APP_NAME, UA},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
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
            Ok(tungstenite::Message::Close(_)) => return Disconnected,
            Err(e @ tungstenite::error::Error::Io(_)) => return IoErr(e),
            Err(e) => return Unknown(e.to_string()),
            _ => continue,
        };

        tokio::spawn(async move {
            let body = serde_json::from_str::<StreamBody>(&message)?;

            match body.r#type.as_str() {
                "friend-online" | "friend-location" => {
                    let content = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?;
                    FRIENDS.write(|friends| friends.update(content)).await;
                }

                "friend-update" => {
                    let user =
                        serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                    FRIENDS.write(|friends| friends.update(user)).await;
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

                    if new_friend.location != "offline" {
                        if let Status::AskMe | Status::Busy = new_friend.status {
                            new_friend.undetermined = true;
                        }
                        FRIENDS.write(|friends| friends.update(new_friend)).await;
                    }
                }

                "friend-offline" | "friend-delete" | "friend-active" => {
                    let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                    FRIENDS.write(|friends| friends.del(&id)).await;
                }

                "user-update" => {
                    let user =
                        serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                    MYSELF.insert(user).await;
                }

                "user-location" => {
                    let user =
                        serde_json::from_str::<FriendOnlineEventContent>(&body.content)?.into();
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
