use crate::general::find_matched_data;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Notification {
    created_at: String,
    details: String, // NotificationDetailInvite, NotificationDetailInviteResponse, NotificationDetailRequestInvite, NotificationDetailRequestInviteResponse, NotificationDetailVoteToKick
    id: String,
    message: String,
    seen: bool,
    receiverUserId: String,
    senderUserId: String,
    r#type: String,
}

#[derive(Serialize)]
enum Response {
    Success { notifications: Vec<Notification> },
    Error { error: String },
}

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

#[post("/friend_request", data = "<req>")]
pub(crate) async fn api_notifications(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(notifications) => Response::Success { notifications },
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<Vec<Notification>> {
    let matched = find_matched_data(req)?;

    let res = reqwest::Client::new()
        .get(URL)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let deserialized: Vec<Notification> = res.json().await?;
        Ok(deserialized)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
