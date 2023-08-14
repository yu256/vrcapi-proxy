use crate::{
    consts::{COOKIE, INVALID_INPUT, UA, UA_VALUE},
    general::find_matched_data,
    CLIENT,
};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success,
    Error(String),
}

#[post("/friend_accept", data = "<req>")]
pub(crate) async fn api_friend_accept(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(_) => (Status::Ok, Json(Response::Success)),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<()> {
    let (auth, id) = req.split_once(':').context(INVALID_INPUT)?;

    let matched = find_matched_data(auth)?;

    let res = CLIENT
        .put(&format!(
            "https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"
        ))
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("{}", res.status())
    }
}
