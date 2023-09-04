use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, into_err, split_colon};
use anyhow::Result;
use rocket::{http::Status, serde::json::Json};

#[post("/friend_request", data = "<req>")]
pub(crate) fn api_friend_request(req: &str) -> (Status, Json<ApiResponse<bool>>) {
    match fetch(req) {
        Ok(_) => (Status::Ok, Json(true.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

fn fetch(req: &str) -> Result<()> {
    split_colon!(req, [auth, user, method]);

    let (_, token) = find_matched_data(auth)?;

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
        &token,
    )?;

    Ok(())
}
