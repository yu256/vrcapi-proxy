use super::utils::find_matched_data;
use crate::{
    api::{response::ApiResponse, utils::request_json},
    split_colon,
};
use anyhow::{anyhow, Result};
use serde_json::json;
const URL: &str = "https://api.vrchat.cloud/api/1/favorites";

#[post("/favorites", data = "<req>")]
pub(crate) fn api_add_favorites(req: &str) -> ApiResponse<bool> {
    fetch(req).into()
}

fn fetch(req: &str) -> Result<bool> {
    split_colon!(req, [auth, r#type, id, tag]);

    let (_, token) = find_matched_data(auth)?;

    request_json(
        "POST",
        URL,
        &token,
        json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ),
    )
    .map_err(|e| match *e {
        ureq::Error::Status(400, _) => {
            anyhow!("既に登録されています。")
        }
        ureq::Error::Status(_, res) => {
            anyhow!(res.into_string().unwrap_or_else(|e| e.to_string()))
        }
        _ => e.into(),
    })?;

    Ok(true)
}
