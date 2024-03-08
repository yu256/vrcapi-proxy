use super::utils::request;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    user_id: String,
    r#type: Type,
}

#[derive(serde::Deserialize)]
enum Type {
    Request,
    Delete,
}

pub(crate) async fn api_friend_request(
    Json(Query {
        auth,
        user_id,
        r#type,
    }): Json<Query>,
) -> Result<bool> {
    let token = validate(auth)?.await;

    let method = match r#type {
        Type::Request => "POST",
        Type::Delete => "DELETE",
    };

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{user_id}/friendRequest"),
        &token,
    )
    .map(|_| true)
}
