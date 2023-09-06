use super::user::User;
use crate::consts::{INVALID_AUTH, VRC_P};
use anyhow::Context as _;
use rocket::tokio::sync::RwLock;
use serde::Serialize;
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResFriend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
    undetermined: bool,
}

impl From<&User> for ResFriend {
    fn from(user: &User) -> Self {
        Self {
            currentAvatarThumbnailImageUrl: user.get_img(),
            id: user.id.to_owned(),
            status: user.status.to_owned(),
            location: user.location.to_owned(),
            undetermined: user.undetermined,
        }
    }
}

impl User {
    pub(crate) fn get_img(&self) -> String {
        let img = match self.tags.iter().any(|tag| tag == VRC_P) {
            true if !self.userIcon.is_empty() => &self.userIcon,
            true if !self.profilePicOverride.is_empty() => &self.profilePicOverride,
            _ => &self.currentAvatarThumbnailImageUrl,
        };
        img.to_owned()
    }
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> anyhow::Result<Vec<ResFriend>> {
    let read = FRIENDS.read().await;
    let friends = read.get(req).context(INVALID_AUTH)?;

    let mut friends = friends.iter().map(ResFriend::from).collect::<Vec<_>>();

    friends.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(friends)
}
