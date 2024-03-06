use super::utils::request;
use crate::{notification::Notification, validate};
use anyhow::Result;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

pub(crate) async fn api_notifications(auth: String) -> Result<Vec<Notification>> {
    let token = validate::validate(&auth)?.await;
    request("GET", URL, &token)?.into_json().map_err(From::from)
}
