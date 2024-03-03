use super::error::WSError;
use crate::global::{AUTHORIZATION, FRIENDS, MYSELF, STREAM_SENDERS};
use crate::user_impl::{Status, User, VecUserExt as _};
use crate::{
    api::request,
    global::{APP_NAME, UA},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use futures::StreamExt as _;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{connect_async, tungstenite};
use trie_match::trie_match;
use WSError::*;

static IS_DISCONNECTED: AtomicBool = AtomicBool::new(true);

pub(crate) async fn stream() -> WSError {
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

    let (tx, mut rx) = mpsc::channel(1);

    let handle = tokio::spawn(async move {
        let (mut stream, _) = match connect_async(req).await {
            Ok(ok) => ok,
            Err(e) => {
                return tx
                    .send(if let tungstenite::error::Error::Io(e) = e {
                        Other(e.to_string())
                    } else {
                        Unknown(e.to_string())
                    })
                    .await;
            }
        };

        while let Some(message) = stream.next().await {
            let message = match message {
                Ok(tungstenite::Message::Text(message)) => message,
                Ok(tungstenite::Message::Ping(_)) => {
                    IS_DISCONNECTED.store(false, Release);
                    continue;
                }
                Ok(tungstenite::Message::Close(_)) => return tx.send(Disconnected).await,
                Err(e) => return tx.send(Other(e.to_string())).await,
                _ => continue,
            };

            if message.starts_with(r#"{"err"#) {
                return tx
                    .send(if !message.contains("authToken") {
                        Unknown(message)
                    } else {
                        Token
                    })
                    .await;
            }

            tokio::spawn(async move {
                let body = serde_json::from_str::<StreamBody>(&message)?;

                trie_match! {
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
                                "GET",
                                &format!("https://api.vrchat.cloud/api/1/users/{id}"),
                                &AUTHORIZATION.1.read().await,
                            )?
                            .into_json::<User>()?;

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
                            let user = serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                            MYSELF.insert(user).await;
                        }

                        "user-location" => {
                            let user = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?.into();
                            MYSELF.insert(user).await;
                        }

                        _ => {}
                    }
                }

                for (_, sender) in STREAM_SENDERS.lock().await.iter_mut() {
                    sender.send(message.clone()).await?;
                }

                Ok::<(), anyhow::Error>(())
            });
        }
        tx.send(Disconnected).await
    });

    let mut interval = tokio::time::interval(Duration::from_secs(60));

    interval.tick().await;

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => break msg,

            _ = interval.tick() => {
                if IS_DISCONNECTED.swap(true, Acquire) {
                    break if handle.is_finished() {
                        match handle.await {
                            Ok(Err(SendError(err))) => err,
                            Err(e) => Other(e.to_string()),
                            Ok(Ok(())) => unreachable!(),
                        }
                    } else {
                        handle.abort();
                        Disconnected
                    }
                }
            }
        }
    }
}
