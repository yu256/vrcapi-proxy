use crate::general::find_matched_data;
use anyhow::{bail, Result};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Friend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
    tags: Vec<String>,
    userIcon: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResFriend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
}

impl From<Friend> for ResFriend {
    fn from(friend: Friend) -> Self {
        ResFriend {
            currentAvatarThumbnailImageUrl: if friend
                .tags
                .iter()
                .any(|tag| tag == "system_supporter")
            {
                friend.userIcon
            } else {
                friend.currentAvatarThumbnailImageUrl
            },
            id: friend.id,
            status: friend.status,
            location: friend.location,
        }
    }
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Vec<ResFriend>),
    Error(String),
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> Json<Response> {
    let result = match fetch(req).await {
        Ok(friends) => Response::Success(friends),
        Err(error) => Response::Error(error.to_string()),
    };

    Json(result)
}

async fn fetch(req: &str) -> Result<Vec<ResFriend>> {
    let matched = find_matched_data(req)?;

    let res = reqwest::Client::new()
        .get(URL)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let deserialized: Vec<Friend> = res.json().await?;
        Ok(modify_friends(deserialized, &matched.askme))
    } else {
        bail!("Error: {}", res.status())
    }
}

fn modify_friends(friends: Vec<Friend>, askme: &bool) -> Vec<ResFriend> {
    let mut friends = friends
        .into_iter()
        .filter(|friend| friend.location != "offline" && (*askme || friend.status != "ask me"))
        .map(ResFriend::from)
        .collect::<Vec<_>>();
    friends.sort_by(|a, b| a.id.cmp(&b.id));
    friends
}
