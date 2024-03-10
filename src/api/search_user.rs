use crate::{
    fetcher::{request, ResponseExt as _},
    validate::validate,
};
use anyhow::Result;
use axum::Json;
use hyper::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
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

    let username = utf8_percent_encode(&username, NON_ALPHANUMERIC).to_string();

    request(
        Method::GET,
        &format!("https://api.vrchat.cloud/api/1/users?search={username}&n={n}"),
        &token,
    )
    .await?
    .json()
    .await
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
    #[serde(skip_serializing_if = "str::is_empty")]
    #[serde(default)]
    userIcon: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    #[serde(default)]
    profilePicOverride: String,
}
