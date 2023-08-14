use crate::consts::{COOKIE, UA, UA_VALUE};
use reqwest::Response;
use std::sync::LazyLock;

pub(crate) static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(crate) async fn request(
    method: reqwest::Method,
    target: &str,
    cookie: &str,
) -> anyhow::Result<Response> {
    Ok(CLIENT
        .request(method, target)
        .header(UA, UA_VALUE)
        .header(COOKIE, cookie)
        .send()
        .await?)
}
