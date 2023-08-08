use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) bio: String,
    pub(crate) bioLinks: Vec<String>,
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) isFriend: bool,
    pub(crate) last_activity: Option<String>,
    pub(crate) location: String,
    pub(crate) status: String,
    pub(crate) statusDescription: String,
}

#[derive(Serialize)]
enum Response {
    Success { user: User },
    Error { error: String },
}

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(user) => Response::Success { user },
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<User> {
    let (auth, user) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .get(&format!("{}{}", URL, user))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let user: User = res.json().await?;
        Ok(user)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
