use crate::general::find_matched_data;
use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
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
    rank: Option<String>,
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
        let mut user: User = res.json().await?;
        add_rank(&mut user);
        Ok(user)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}

fn add_rank(user: &mut User) {
    for tag in user.tags.iter().rev() {
        match tag.as_str() {
            "system_trust_veteran" => {
                user.rank = Some("Trusted".to_string());
                break;
            }
            "system_trust_trusted" => {
                user.rank = Some("Known".to_string());
                break;
            }
            "system_trust_known" => {
                user.rank = Some("User".to_string());
                break;
            }
            "system_trust_basic" => {
                user.rank = Some("New User".to_string());
                break;
            }
            "system_troll" => {
                user.rank = Some("Troll".to_string());
                break;
            }
            _ => {}
        }
    }

    user.tags.clear()
}
