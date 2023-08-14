use crate::{
    consts::{COOKIE, UA, UA_VALUE, VRC_P},
    general::find_matched_data,
    CLIENT,
};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Friend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
    tags: Vec<String>,
    userIcon: String,
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResFriend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
}

impl From<Friend> for ResFriend {
    fn from(friend: Friend) -> Self {
        ResFriend {
            currentAvatarThumbnailImageUrl: if !friend.tags.iter().any(|tag| tag == VRC_P) {
                friend.currentAvatarThumbnailImageUrl
            } else if friend.userIcon.is_empty() {
                friend.profilePicOverride
            } else {
                friend.userIcon
            },
            id: friend.id,
            status: friend.status,
            location: friend.location,
        }
    }
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Vec<ResFriend>),
    Error(String),
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(friends) => (Status::Ok, Json(Response::Success(friends))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<Vec<ResFriend>> {
    let matched = find_matched_data(req)?;

    let res = CLIENT
        .get(URL)
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let deserialized: Vec<Friend> = res.json().await?;
        Ok(modify_friends(deserialized, &matched.askme))
    } else {
        bail!("{}", res.status())
    }
}

fn modify_friends(friends: Vec<Friend>, askme: &bool) -> Vec<ResFriend> {
    let mut friends = friends
        .into_iter()
        .filter(|friend| friend.location != "offline" && (*askme || friend.status != "ask me"))
        .map(ResFriend::from)
        .collect::<Vec<_>>();
    friends.sort_by(|a, b| a.id.cmp(&b.id));
    friends
}
