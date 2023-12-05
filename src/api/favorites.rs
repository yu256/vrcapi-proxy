use super::request;
use crate::global::FAVORITE_FRIENDS;
use crate::{api::utils::request_json, split_colon};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Favorite {
    // id: String,
    // r#type: String,
    favoriteId: String,
    // tags: Vec<String>,
}

pub(crate) async fn api_add_favorites(
    mut req: std::str::Split<'_, char>,
    token: &str,
) -> Result<bool> {
    split_colon!(req, [r#type, id, tag]);

    request_json(
        "POST",
        "https://api.vrchat.cloud/api/1/favorites",
        token,
        json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ),
    )
    .map(|_| true)
}

pub(crate) async fn api_re_fetch(_req: std::str::Split<'_, char>, token: &str) -> Result<bool> {
    fetch_favorite_friends(token).await.map(|_| true)
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
