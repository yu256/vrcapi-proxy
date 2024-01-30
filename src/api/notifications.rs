use super::utils::request;
use crate::{validate, notification::Notification};
use anyhow::Result;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

pub(crate) async fn api_notifications(req: String) -> Result<Vec<Notification>> {
    validate!(req, token);
    request("GET", URL, token)?.into_json().map_err(From::from)
}
