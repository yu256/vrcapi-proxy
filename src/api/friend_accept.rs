use super::utils::request;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;
use hyper::Method;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    notification_id: String,
}

pub(crate) async fn api_friend_accept(
    Json(Query {
        auth,
        notification_id,
    }): Json<Query>,
) -> Result<()> {
    let token = validate(auth)?.await;

    request(
        Method::PUT,
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{notification_id}/accept"),
        &token,
    )
    .await?;

    Ok(())
}
