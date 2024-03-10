use crate::{fetcher::request, validate::validate};
use anyhow::Result;
use axum::Json;
use hyper::Method;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    instance_id: String, //wrldも含む(worldId:instanceId)
}

pub(crate) async fn api_invite_myself(
    Json(Query { auth, instance_id }): Json<Query>,
) -> Result<bool> {
    let token = validate(auth)?.await;

    request(
        Method::POST,
        &format!("https://api.vrchat.cloud/api/1/invite/myself/to/{instance_id}"),
        &token,
    )
    .await
    .map(|_| true)
}
