use super::request;
use crate::api::utils::request_json;
use crate::global::FAVORITE_FRIENDS;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    favorite_type: String, // group_[0..=3] | avatars[1..=4] | worlds[1..=4]
    favorite_id: String,
    tags: Vec<String>,
}

pub(crate) async fn api_add_favorites(
    Json(Query {
        auth,
        favorite_type,
        favorite_id,
        tags,
    }): Json<Query>,
) -> Result<bool> {
    let token = validate(auth)?.await;

    request_json(
        "POST",
        "https://api.vrchat.cloud/api/1/favorites",
        &token,
        json!( {"type": favorite_type, "favoriteId": favorite_id, "tags": tags} ),
    )
    .map(|_| true)
}

pub(crate) async fn api_re_fetch(auth: String) -> Result<bool> {
    let token = validate(auth)?.await;
    fetch_favorite_friends(&token).await.map(|_| true)
}

pub(crate) async fn fetch_favorite_friends(token: &str) -> Result<()> {
    *FAVORITE_FRIENDS.write().await = request(
        "GET",
        "https://api.vrchat.cloud/api/1/favorites?type=friend&n=60",
        token,
    )?
    .into_json::<Vec<Favorite>>()?
    .into_iter()
    .map(|favorite| favorite.favoriteId)
    .collect();

    Ok(())
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Favorite {
    // id: String,
    // r#type: String,
    favoriteId: String,
    // tags: Vec<String>,
}
