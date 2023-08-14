use super::utils::request;
use crate::{consts::INVALID_INPUT, general::find_matched_data};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct ResStatus {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResStatus),
    Error(String),
}

#[post("/friend_status", data = "<req>")]
pub(crate) async fn api_friend_status(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(Response::Success(status))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<ResStatus> {
    let (auth, user) = req.split_once(':').context(INVALID_INPUT)?;

    let matched = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendStatus"),
        &matched.token,
    )
    .await?;

    if res.status().is_success() {
        let status: ResStatus = res.json().await?;
        Ok(status)
    } else {
        bail!("{}", res.status())
    }
}
