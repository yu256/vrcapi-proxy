use super::utils::request;
use crate::consts::VRC_P;
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static FRIENDS: LazyLock<Mutex<HashMap<String, Vec<Friend>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false";

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct Friend {
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) id: String,
    pub(crate) status: String,
    pub(crate) location: String,
    pub(crate) tags: Vec<String>,
    pub(crate) userIcon: String,
    pub(crate) profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResFriend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
}

impl From<&Friend> for ResFriend {
    fn from(friend: &Friend) -> Self {
        let img = match friend.tags.iter().any(|tag| tag == VRC_P) {
            true if !friend.userIcon.is_empty() => &friend.userIcon,
            true if !friend.profilePicOverride.is_empty() => &friend.profilePicOverride,
            _ => &friend.currentAvatarThumbnailImageUrl,
        };

        ResFriend {
            currentAvatarThumbnailImageUrl: img.to_owned(),
            id: friend.id.to_owned(),
            status: friend.status.to_owned(),
            location: friend.location.to_owned(),
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
    match FRIENDS.lock().await.get(req) {
        Some(friends) => (Status::Ok, Json(Response::Success(modify_friends(friends)))),

        None => (
            Status::InternalServerError,
            Json(Response::Error("failed to auth.".to_string())),
        ),
    }
}

pub(crate) async fn fetch_friends(token: &str) -> Result<Vec<Friend>> {
    let res = request(reqwest::Method::GET, URL, token).await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}

fn modify_friends(friends: &Vec<Friend>) -> Vec<ResFriend> {
    let mut friends = friends
        .iter()
        .filter(|friend| friend.location != "offline")
        .map(ResFriend::from)
        .collect::<Vec<_>>();
    friends.sort_by(|a, b| a.id.cmp(&b.id));
    friends
}
