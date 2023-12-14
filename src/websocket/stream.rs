use super::error::WSError;
use crate::global::{AUTHORIZATION, FRIENDS, USERS};
use crate::try_;
use crate::user_impl::{Status, User, VecUserExt as _};
use crate::{
    api::request,
    global::{APP_NAME, UA},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use futures::StreamExt as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest as _};
use trie_match::trie_match;
use WSError::*;

pub(crate) async fn stream() -> WSError {
    let mut req = try_!(format!(
        "wss://pipeline.vrchat.cloud/?{}",
        &*AUTHORIZATION.1.read().await
    )
    .into_client_request());
    req.headers_mut()
        .insert(UA, HeaderValue::from_static(APP_NAME));

    let (stream, _) = try_!(connect_async(req).await);

    let (_, mut read) = stream.split();

    while let Some(message) = read.next().await {
        let message = match message {
            Ok(message) if message.is_ping() => continue,
            Ok(message) => message.to_string(),
            Err(e) => return OtherError(e.to_string()),
        };

        if message.starts_with(r#"{"err":"authToken"#) {
            return TokenError;
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
                        USERS.insert(user).await;
                    }

                    "user-location" => {
                        let user = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?.into();
                        USERS.insert(user).await;
                    }
                    _ => {}
                }
            }

            Ok::<(), anyhow::Error>(())
        });
    }

    OtherError("disconnected".into())
}
