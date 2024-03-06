use super::utils::request;
use crate::api::utils::request_json;
use crate::global::{FRIENDS, MYSELF};
use crate::user_impl::{Status, User};
use crate::validate;
use anyhow::{anyhow, Context, Result};
use axum::Json;
use serde::{Deserialize, Serialize};
use trie_match::trie_match;

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    id: String,
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    travelingToLocation: Option<String>,
    status: Status,
    statusDescription: String,
    rank: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    userIcon: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    profilePicOverride: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        let mut rank = user
            .tags
            .iter()
            .rev()
            .find_map(|tag| {
                trie_match! {
                    match tag.as_str() {
                        "system_trust_veteran" => Some("Trusted"),
                        "system_trust_trusted" => Some("Known"),
                        "system_trust_known" => Some("User"),
                        "system_trust_basic" => Some("New User"),
                        "system_troll" => Some("Troll"),
                        _ => None,
                    }
                }
            })
            .unwrap_or("Visitor")
            .to_owned();

        if user.tags.iter().any(|tag| tag == "system_supporter") {
            rank += " VRC+"
        }

        ResUser {
            id: user.id,
            currentAvatarThumbnailImageUrl: user.currentAvatarThumbnailImageUrl,
            bio: user.bio,
            bioLinks: user.bioLinks,
            displayName: user.displayName,
            isFriend: user.isFriend,
            location: user.location,
            travelingToLocation: user.travelingToLocation,
            status: user.status,
            statusDescription: user.statusDescription,
            rank,
            userIcon: user.userIcon,
            profilePicOverride: user.profilePicOverride,
        }
    }
}

pub(crate) async fn api_user(req: String) -> Result<ResUser> {
    let mut iter = req.split(':');
    let token = validate::validate(iter.next().context(crate::global::INVALID_REQUEST)?)?.await;
    match (iter.next(), iter.next()) {
		(None, None) => match MYSELF.read().await {
            Some(mut user) => Ok({
                user.unsanitize();
                user.into()
            }),
            None => Err(anyhow!("プロフィールの取得に失敗しました。トークンが無効か、ユーザー情報の取得が完了していません。後者の場合は、オンラインになると取得されます。")),
        }
		(Some(user), force) => {
			if force != Some("true") && let Some(user) = FRIENDS
			.read(|friends| friends.iter().find(|u| u.id == user).cloned())
			.await
		{
			return Ok(user.into());
		}

            match request("GET", &format!("{URL}{user}"), &token)?.into_json::<User>() {
                Ok(mut json) => Ok({
                    json.unsanitize();
                    json.into()
                }),
                Err(err) => Err(err.into()),
            }
        }
        (None, Some(_)) => unsafe { std::hint::unreachable_unchecked() } // イテレータからNoneの次にSomeが返ってくることはない
    }
}

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
    let token = validate::validate(&req.auth)?.await;

    request_json(
        "PUT",
        &format!("{URL}{}", req.user),
        &token,
        req.query.clone(),
    )?;

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
