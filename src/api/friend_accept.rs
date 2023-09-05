use super::{
    response::ApiResponse,
    utils::{find_matched_data, request},
};
use crate::split_colon;
use anyhow::Result;

#[post("/friend_accept", data = "<req>")]
pub(crate) fn api_friend_accept(req: &str) -> ApiResponse<()> {
    fetch(req).into()
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
