use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::Serialize;

#[derive(Serialize)]
enum Response {
    Success {},
    Error { error: String },
}

#[post("/friend_accept", data = "<req>")]
pub(crate) async fn api_friend_accept(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(_) => Response::Success {},
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<()> {
    let (auth, id) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let url = format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept");

    let res = reqwest::Client::new()
        .put(&url)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
