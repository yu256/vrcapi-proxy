use crate::{fetcher::request, validate::validate};
use anyhow::Result;
use axum::Json;
use hyper::Method;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    user_id: String,
    method: HTTPMethod,
}

#[derive(serde::Deserialize)]
enum HTTPMethod {
    Request,
    Delete,
}

pub(crate) async fn api_friend_request(
    Json(Query {
        auth,
        user_id,
        method,
    }): Json<Query>,
) -> Result<bool> {
    let token = validate(auth)?.await;

    let method = match method {
        HTTPMethod::Request => Method::POST,
        HTTPMethod::Delete => Method::DELETE,
    };

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{user_id}/friendRequest"),
        &token,
    )
    .await
    .map(|_| true)
}
