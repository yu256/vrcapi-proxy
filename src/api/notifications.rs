use super::utils::{request, ResponseExt as _};
use crate::{notification::Notification, validate::validate};
use anyhow::Result;
use hyper::Method;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

pub(crate) async fn api_notifications(auth: String) -> Result<Vec<Notification>> {
    let token = validate(auth)?.await;
    request(Method::GET, URL, &token)
        .await?
        .json()
        .await
}
