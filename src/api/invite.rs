use super::request;
use crate::{api::utils::find_matched_data, global::INVALID_REQUEST};
use anyhow::{Context, Result};

const URL: &str = "https://api.vrchat.cloud/api/1/invite/myself/to/";

pub(crate) async fn api_invite_myself(req: String) -> Result<bool> {
    let (auth, instance_id) = req.split_once(':').context(INVALID_REQUEST)?;
    let url = format!("{}{}", URL, instance_id);
    let token = find_matched_data(auth)?.1;
    request("POST", &url, &token).map(|_| true)
}
