use super::utils::{find_matched_data, request};
use crate::{into_err, split_colon};
use anyhow::Result;
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum ApiResponse {
    Success,
    Error(String),
}

#[post("/friend_accept", data = "<req>")]
pub(crate) fn api_friend_accept(req: &str) -> (Status, Json<ApiResponse>) {
    match fetch(req) {
        Ok(_) => (Status::Ok, Json(ApiResponse::Success)),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

fn fetch(req: &str) -> Result<()> {
    split_colon!(req, [auth, id]);

    let (_, token) = find_matched_data(auth)?;

    request(
        "PUT",
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"),
        &token,
    )?;

    Ok(())
}
