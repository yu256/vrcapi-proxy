use crate::general::find_matched_data;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Friend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
}

#[derive(Serialize)]
enum Response {
    Success { friends: Vec<Friend> },
    Error { error: String },
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(friends) => Response::Success { friends },
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<Vec<Friend>> {
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
        bail!("Error: status code: {}", res.status())
    }
}

fn modify_friends(friends: Vec<Friend>, askme: &bool) -> Vec<Friend> {
    let mut friends = friends
        .into_iter()
        .filter(|friend| friend.location != "offline" && (*askme || friend.status != "ask me"))
        .collect::<Vec<_>>();
    friends.sort_by(|a, b| a.id.cmp(&b.id));
    friends
}
