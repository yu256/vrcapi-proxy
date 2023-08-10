use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users?search=";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
    tags: Vec<String>,
    userIcon: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct ResUser {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        ResUser {
            currentAvatarThumbnailImageUrl: if user.tags.iter().any(|tag| tag == "system_supporter")
            {
                user.userIcon
            } else {
                user.currentAvatarThumbnailImageUrl
            },
            displayName: user.displayName,
            id: user.id,
            isFriend: user.isFriend,
            statusDescription: user.statusDescription,
        }
    }
}

#[derive(Serialize)]
enum Response {
    Success(Vec<ResUser>),
    Error(String),
}

#[post("/search_user", data = "<req>")]
pub(crate) async fn api_search_user(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(users) => Response::Success(users),
        Err(error) => Response::Error(error.to_string()),
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<Vec<ResUser>> {
    let (auth, user) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .get(&format!("{}{}", URL, user))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let users: Vec<User> = res.json().await?;
        Ok(users.into_iter().map(ResUser::from).collect())
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
