use super::utils::request;
use crate::split_colon;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    #[serde(default)]
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    #[serde(default)]
    statusDescription: String,
    #[serde(default)]
    userIcon: String,
    #[serde(default)]
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    userIcon: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    profilePicOverride: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        ResUser {
            currentAvatarThumbnailImageUrl: user.currentAvatarThumbnailImageUrl,
            displayName: user.displayName,
            id: user.id,
            isFriend: user.isFriend,
            statusDescription: user.statusDescription,
            userIcon: user.userIcon,
            profilePicOverride: user.profilePicOverride,
        }
    }
}

pub(crate) async fn api_search_user(
    mut req: std::str::Split<'_, char>,
    token: &str,
) -> Result<Vec<ResUser>> {
    split_colon!(req, [user]);

    match request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/users?search={}&n=100", user),
        token,
    )?
    .into_json::<Vec<User>>()
    {
        Ok(user) => Ok(user.into_iter().map(ResUser::from).collect()),
        Err(err) => Err(err.into()),
    }
}
