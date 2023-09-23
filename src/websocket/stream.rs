#![allow(clippy::redundant_closure_call)]

use crate::global::FRIENDS;
use crate::websocket::structs::VecUserExt as _;
use crate::websocket::User;
use crate::{
    api::request,
    global::{UA, UA_VALUE},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use anyhow::{anyhow, Result};
use futures::StreamExt as _;
use rocket::tokio;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest as _};

macro_rules! write_friends {
    ($FRIENDS:expr, $data:expr, $fun:expr) => {{
        let mut unlocked = $FRIENDS.write().await;
        if let Some(friends) = unlocked.get_mut(&$data.0) {
            $fun(friends);
        }
    }}
}

pub(crate) async fn stream(data: Arc<(String, String)>) -> Result<()> {
    let mut req = format!("wss://pipeline.vrchat.cloud/?{}", &data.1).into_client_request()?;
    let headers = req.headers_mut();
    headers.insert(UA, UA_VALUE.try_into()?);

    let (stream, _) = connect_async(req).await?;

    let (_, mut read) = stream.split();

    while let Some(message) = read.next().await {
        let message = message?.to_string();
        if message.starts_with(r#"{"err":"authToken"#) {
            return Ok(());
        }
        let data = Arc::clone(&data);

        tokio::spawn(async move {
            let body = serde_json::from_str::<StreamBody>(&message)?;
            match body.r#type.as_str() {
                "friend-online" | "friend-location" => {
                    if let Ok(content) =
                        serde_json::from_str::<FriendOnlineEventContent>(&body.content)
                    {
                        write_friends!(FRIENDS, data, |friends: &mut Vec<User>| {
                            friends.update(content)
                        });
                    } else {
                        eprintln!("not deserialized: {message}"); // debug
                    }
                }

                "friend-update" => {
                    if let Ok(content) =
                        serde_json::from_str::<FriendUpdateEventContent>(&body.content)
                    {
                        write_friends!(FRIENDS, data, |friends: &mut Vec<User>| {
                            friends.update(content.user)
                        });
                    } else {
                        eprintln!("not deserialized: {message}"); // debug
                    }
                }

                "friend-add" => {
                    let content = serde_json::from_str::<UserIdContent>(&body.content)?;
                    let mut new_friend = request(
                        "GET",
                        &format!("https://api.vrchat.cloud/api/1/users/{}", content.userId),
                        &data.1,
                    )?
                        .into_json::<User>()?;

                    if new_friend.location != "offline" {
                        if new_friend.status == "ask me" || new_friend.status == "busy" {
                            new_friend.undetermined = true;
                        }
                        write_friends!(FRIENDS, data, |friends: &mut Vec<User>| {
                            friends.update(new_friend)
                        });
                    }
                }

                "friend-offline" | "friend-delete" | "friend-active" => {
                    if let Ok(content) = serde_json::from_str::<UserIdContent>(&body.content) {
                        write_friends!(FRIENDS, data, |friends: &mut Vec<User>| {
                            friends.retain(|f| f.id != content.userId)
                        });
                    } else {
                        eprintln!("not deserialized: {message}"); // debug
                    }
                }
                _ => {}
            }

            Ok::<(), anyhow::Error>(())
        });
    }

    Err(anyhow!("disconnected"))
}
