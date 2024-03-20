use crate::global::{FAVORITE_FRIENDS, FRIENDS, WS_HANDLER};
use crate::user::{Status, User};
use crate::validate::validate;
use anyhow::{ensure, Result};
use serde::Serialize;

pub(crate) async fn api_friends(auth: String) -> Result<ResFriend> {
    drop(validate(auth)?);

    ensure!(
        WS_HANDLER.read().await.is_some(),
        "WebSocketに接続されていません。"
    );

    let (public, private) = FRIENDS
        .read()
        .await
        .online
        .iter()
        .map(Friend::from)
        .partition(|friend| friend.location != "private");

    Ok(ResFriend { public, private })
}

pub(crate) async fn api_friends_filtered(auth: String) -> Result<ResFriend> {
    let favorites = FAVORITE_FRIENDS.read().await;
    api_friends(auth).await.map(|mut friends| {
        let fun = |friend: &Friend| favorites.contains(&friend.id);
        friends.private.retain(fun);
        friends.public.retain(fun);
        friends
    })
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Friend {
    currentAvatarThumbnailImageUrl: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    userIcon: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    profilePicOverride: String,
    id: String,
    status: Status,
    location: String,
}

#[derive(Serialize)]
pub(crate) struct ResFriend {
    public: Vec<Friend>,
    private: Vec<Friend>,
}

impl From<&User> for Friend {
    fn from(user: &User) -> Self {
        Self {
            currentAvatarThumbnailImageUrl: user.currentAvatarThumbnailImageUrl.clone(),
            userIcon: user.userIcon.clone(),
            profilePicOverride: user.profilePicOverride.clone(),
            id: user.id.clone(),
            status: user.status,
            location: user.location.clone(),
        }
    }
}
