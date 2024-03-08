use super::utils::request;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;
use serde::{Deserialize, Serialize};

const MAX: usize = 100;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    username: String,
    n: Option<usize>,
}

pub(crate) async fn api_search_user(
    Json(Query { auth, username, n }): Json<Query>,
) -> Result<Vec<HitUser>> {
    let token = validate(auth)?.await;

    let n = n.filter(|&n| n != 0 && n <= MAX).unwrap_or(MAX);

    request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/users?search={username}&n={n}"),
        &token,
    )?
    .into_json()
    .map_err(From::from)
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub(crate) struct HitUser {
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
