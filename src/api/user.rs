use crate::{
    consts::{COOKIE, INVALID_INPUT, UA, UA_VALUE, VRC_P},
    general::find_matched_data,
    CLIENT,
};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
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
    userIcon: String,
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
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
pub(crate) enum Response {
    Success(ResUser),
    Error(String),
}

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(user) => (Status::Ok, Json(Response::Success(user))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<ResUser> {
    let (auth, user) = req.split_once(':').context(INVALID_INPUT)?;

    let matched = find_matched_data(auth)?;

    let res = CLIENT
        .get(&format!("{}{}", URL, user))
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let user: User = res.json().await?;
        Ok(add_rank(user))
    } else {
        bail!("Error: {}", res.status())
    }
}

fn add_rank(user: User) -> ResUser {
    let mut rank = None;
    for tag in user.tags.iter().rev() {
        match tag.as_str() {
            "system_trust_veteran" => {
                rank = Some("Trusted");
                break;
            }
            "system_trust_trusted" => {
                rank = Some("Known");
                break;
            }
            "system_trust_known" => {
                rank = Some("User");
                break;
            }
            "system_trust_basic" => {
                rank = Some("New User");
                break;
            }
            "system_troll" => {
                rank = Some("Troll");
                break;
            }
            _ => {}
        }
    }

    ResUser {
        bio: user.bio,
        bioLinks: user.bioLinks,
        currentAvatarThumbnailImageUrl: if !user.tags.iter().any(|tag| tag == VRC_P) {
            user.currentAvatarThumbnailImageUrl
        } else if user.userIcon.is_empty() {
            user.profilePicOverride
        } else {
            user.userIcon
        },
        displayName: user.displayName,
        last_activity: user.last_activity,
        location: user.location,
        status: user.status,
        statusDescription: user.statusDescription,
        rank: rank.unwrap_or_else(|| "Visitor").to_owned(),
    }
}
