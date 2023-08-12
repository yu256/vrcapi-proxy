use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    general::find_matched_data,
    CLIENT,
};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Notification {
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
pub(crate) enum Response {
    Success(Vec<Notification>),
    Error(String),
}

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

#[post("/notifications", data = "<req>")]
pub(crate) async fn api_notifications(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(notifications) => (Status::Ok, Json(Response::Success(notifications))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<Vec<Notification>> {
    let matched = find_matched_data(req)?;

    let res = CLIENT
        .get(URL)
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let deserialized: Vec<Notification> = res.json().await?;
        Ok(deserialized)
    } else {
        bail!("Error: {}", res.status())
    }
}
