use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, into_err, split_colon};
use anyhow::Result;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct ResStatus {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

#[post("/friend_status", data = "<req>")]
pub(crate) fn api_friend_status(req: &str) -> (Status, Json<ApiResponse<ResStatus>>) {
    match fetch(req) {
        Ok(status) => (Status::Ok, Json(status.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

fn fetch(req: &str) -> Result<ResStatus> {
    split_colon!(req, [auth, user]);

    let (_, token) = find_matched_data(auth)?;

    request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendStatus"),
        &token,
    )
    .map(|res| res.into_json::<ResStatus>())?
    .map_err(From::from)
}
