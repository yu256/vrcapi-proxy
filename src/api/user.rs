use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    last_activity: Option<String>,
    location: String,
    status: String,
    statusDescription: String,
    tags: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct ResUser {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    last_activity: Option<String>,
    location: String,
    status: String,
    statusDescription: String,
    rank: String,
}

#[derive(Serialize)]
enum Response {
    Success { user: ResUser },
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

async fn fetch(req: &str) -> Result<ResUser> {
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
        Ok(add_rank(user))
    } else {
        bail!("Error: status code: {}", res.status())
    }
}

fn add_rank(user: User) -> ResUser {
    let mut rank = None;
    for tag in user.tags.iter().rev() {
        match tag.as_str() {
            "system_trust_veteran" => {
                rank = Some("Trusted".to_string());
                break;
            }
            "system_trust_trusted" => {
                rank = Some("Known".to_string());
                break;
            }
            "system_trust_known" => {
                rank = Some("User".to_string());
                break;
            }
            "system_trust_basic" => {
                rank = Some("New User".to_string());
                break;
            }
            "system_troll" => {
                rank = Some("Troll".to_string());
                break;
            }
            _ => {}
        }
    }

    ResUser {
        bio: user.bio,
        bioLinks: user.bioLinks,
        currentAvatarThumbnailImageUrl: user.currentAvatarThumbnailImageUrl,
        displayName: user.displayName,
        last_activity: user.last_activity,
        location: user.location,
        status: user.status,
        statusDescription: user.statusDescription,
        rank: rank.unwrap_or_else(|| "Visitor".to_string()),
    }
}
