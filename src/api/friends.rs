use crate::global::{FAVORITE_FRIENDS, FRIENDS};
use crate::validate;
use crate::websocket::structs::Status;
use crate::websocket::User;
use anyhow::Result;
use serde::Serialize;

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
    undetermined: bool,
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
            undetermined: user.undetermined,
        }
    }
}

pub(crate) async fn api_friends(req: String) -> Result<ResFriend> {
    validate!(req);
    let (public, private) = FRIENDS
        .read(|friends| {
            friends
                .iter()
                .map(Friend::from)
                .partition(|friend| friend.location != "private")
        })
        .await;

    Ok(ResFriend { public, private })
}

pub(crate) async fn api_friends_filtered(req: String) -> Result<ResFriend> {
    let favorites = FAVORITE_FRIENDS.read().await;
    api_friends(req).await.map(|mut friends| {
        let fun = |friend: &Friend| favorites.contains(&friend.id);
        crate::struct_foreach!(friends, [private, public], retain(fun));
        friends
    })
}
