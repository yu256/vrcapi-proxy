use super::user::User;
use crate::consts::INVALID_AUTH;
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

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> anyhow::Result<Vec<ResFriend>> {
    let mut friends = {
        let read = FRIENDS.read().await;
        read.get(req)
            .context(INVALID_AUTH)?
            .iter()
            .map(ResFriend::from)
            .collect::<Vec<_>>()
    };

    friends.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(friends)
}
