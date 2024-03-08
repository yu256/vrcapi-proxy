use super::utils::request_json;
use crate::{global::MYSELF, user_impl::Status, validate::validate};
use anyhow::Result;
use axum::Json;
use reqwest::Method;
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

    MYSELF
        .write(|user| {
            user.status = req.query.status;
            user.statusDescription = req.query.statusDescription;
            user.bio = req.query.bio;
            user.bioLinks = req.query.bioLinks;
            if let Some(user_icon) = req.query.userIcon {
                user.userIcon = user_icon;
            }
        })
        .await;

    Ok(true)
}
