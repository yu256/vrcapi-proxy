use super::user::User;
use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::Serialize;

const URL: &str = "https://api.vrchat.cloud/api/1/users?search=";

#[derive(Serialize)]
enum Response {
    Success { users: Vec<User> },
    Error { error: String },
}

#[post("/search_user", data = "<req>")]
pub(crate) async fn api_search_user(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(users) => Response::Success { users },
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<Vec<User>> {
    let (auth, user) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .get(&format!("{}{}", URL, user))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let user: Vec<User> = res.json().await?;
        Ok(user)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
