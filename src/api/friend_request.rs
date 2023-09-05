use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, split_colon};
use anyhow::Result;

#[post("/friend_request", data = "<req>")]
pub(crate) fn api_friend_request(req: &str) -> ApiResponse<bool> {
    fetch(req).into()
}

fn fetch(req: &str) -> Result<bool> {
    split_colon!(req, [auth, user, method]);

    let (_, token) = find_matched_data(auth)?;

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
        &token,
    )?;

    Ok(true)
}
