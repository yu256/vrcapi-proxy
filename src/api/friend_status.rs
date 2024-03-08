use super::utils::request;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    user_id: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct ResStatus {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

pub(crate) async fn api_friend_status(
    Json(Query { auth, user_id }): Json<Query>,
) -> Result<ResStatus> {
    let token = validate(auth)?.await;

    request(
        Method::GET,
        &format!("https://api.vrchat.cloud/api/1/user/{user_id}/friendStatus"),
        &token,
    )
    .await?
    .json()
    .await
    .map_err(From::from)
}
