use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Status {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Status),
    Error(String),
}

#[post("/friend_status", data = "<req>")]
pub(crate) async fn api_friend_status(req: &str) -> Json<Response> {
    let result = match fetch(req).await {
        Ok(status) => Response::Success(status),
        Err(error) => Response::Error(error.to_string()),
    };

    Json(result)
}

async fn fetch(req: &str) -> Result<Status> {
    let (auth, user) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .get(&format!(
            "https://api.vrchat.cloud/api/1/user/{user}/friendStatus"
        ))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let status: Status = res.json().await?;
        Ok(status)
    } else {
        bail!("Error: {}", res.status())
    }
}
