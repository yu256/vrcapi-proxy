use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success,
    Error(String),
}

#[post("/friend_accept", data = "<req>")]
pub(crate) async fn api_friend_accept(req: &str) -> Json<Response> {
    let result = match fetch(req).await {
        Ok(_) => Response::Success,
        Err(error) => Response::Error(error.to_string()),
    };

    Json(result)
}

async fn fetch(req: &str) -> Result<()> {
    let (auth, id) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .put(&format!(
            "https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"
        ))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("Error: {}", res.status())
    }
}
