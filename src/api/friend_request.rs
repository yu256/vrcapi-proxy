use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success,
    Error(String),
}

#[post("/friend_request", data = "<req>")]
pub(crate) async fn api_friend_request(req: &str) -> Json<Response> {
    let result = match fetch(req, true).await {
        Ok(_) => Response::Success,
        Err(error) => Response::Error(error.to_string()),
    };

    Json(result)
}

#[delete("/friend_request", data = "<req>")]
pub(crate) async fn api_del_friend_request(req: &str) -> Json<Response> {
    let result = match fetch(req, false).await {
        Ok(_) => Response::Success,
        Err(error) => Response::Error(error.to_string()),
    };

    Json(result)
}

async fn fetch(req: &str, is_post: bool) -> Result<()> {
    let (auth, user) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let url = format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user);

    let res = if is_post {
        reqwest::Client::new()
            .post(&url)
            .header("User-Agent", "vrc-rs")
            .header("Cookie", &matched.token)
            .send()
            .await?
    } else {
        reqwest::Client::new()
            .delete(&url)
            .header("User-Agent", "vrc-rs")
            .header("Cookie", &matched.token)
            .send()
            .await?
    };

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("Error: {}", res.status())
    }
}
