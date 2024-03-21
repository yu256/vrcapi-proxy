use crate::{fetcher::request_json, global::USERS, user::Status, validate::validate};
use anyhow::Result;
use axum::Json;
use hyper::Method;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileUpdateQuery {
    auth: String,
    user: String,
    query: Query,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
struct Query {
    status: Status,
    statusDescription: String,
    bio: String,
    bioLinks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    userIcon: Option<String>,
}

pub(crate) async fn api_update_profile(Json(req): Json<ProfileUpdateQuery>) -> Result<bool> {
    let token = validate(&req.auth)?.await;

    request_json(
        Method::PUT,
        &format!("{URL}{}", req.user),
        &token,
        req.query.clone(),
    )
    .await?;

    let mut binding = USERS.write().await;
    let locked = binding.myself.as_mut().unwrap();

    locked.status = req.query.status;
    locked.statusDescription = req.query.statusDescription;
    locked.bio = req.query.bio;
    locked.bioLinks = req.query.bioLinks;
    if let Some(user_icon) = req.query.userIcon {
        locked.userIcon = user_icon;
    }

    Ok(true)
}
