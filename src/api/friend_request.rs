use crate::{
    consts::{COOKIE, INVALID_INPUT, UA, UA_VALUE},
    general::find_matched_data,
};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success,
    Error(String),
}

#[post("/friend_request", data = "<req>")]
pub(crate) async fn api_friend_request(req: &str) -> (Status, Json<Response>) {
    match fetch(req, true).await {
        Ok(_) => (Status::Ok, Json(Response::Success)),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

#[delete("/friend_request", data = "<req>")]
pub(crate) async fn api_del_friend_request(req: &str) -> (Status, Json<Response>) {
    match fetch(req, false).await {
        Ok(_) => (Status::Ok, Json(Response::Success)),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str, is_post: bool) -> Result<()> {
    let (auth, user) = req.split_once(':').context(INVALID_INPUT)?;

    let matched = find_matched_data(auth)?;

    let url = format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user);

    let res = if is_post {
        reqwest::Client::new()
            .post(&url)
            .header(UA, UA_VALUE)
            .header(COOKIE, &matched.token)
            .send()
            .await?
    } else {
        reqwest::Client::new()
            .delete(&url)
            .header(UA, UA_VALUE)
            .header(COOKIE, &matched.token)
            .send()
            .await?
    };

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("Error: {}", res.status())
    }
}
