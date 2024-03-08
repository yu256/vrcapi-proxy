use crate::global::{APP_NAME, COOKIE, UA};
use anyhow::{anyhow, Result};
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Deserialize)]
struct ErrorMessage {
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: String,
    // status_code: u32,
}

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(super) enum Header<'a> {
    Cookie(&'a str),
    Auth((&'a str, &'a str)),
}

pub(super) async fn make_request(
    method: Method,
    target: &str,
    header: Header<'_>,
    serializable: Option<impl Serialize>,
) -> Result<Response> {
    let builder = CLIENT.request(method, target).header(UA, APP_NAME);

    let builder = match header {
        Header::Cookie(cookie) => builder.header(COOKIE, cookie),
        Header::Auth((header, value)) => builder.header(header, value),
    };

	let response = if let Some(serializable) = serializable {
        builder.json(&serializable).send()
    } else {
        builder.send()
    }.await;

    match response {
        Ok(response) if response.status().is_success() => Ok(response),
        Ok(response) => Err(anyhow!(
            "{}",
            response.json::<ErrorMessage>().await?
                .error
                .message
                .replace('\"', "")
        )),
        Err(e) => Err(e.into()),
    }
}

#[inline]
pub(crate) async fn request(method: Method, target: &str, cookie: &str) -> Result<Response> {
    make_request(method, target, Header::Cookie(cookie), None::<()>).await
}

#[inline]
pub(crate) async fn request_json(
    method: Method,
    target: &str,
    cookie: &str,
    serializable: impl Serialize,
) -> Result<Response> {
    make_request(method, target, Header::Cookie(cookie), Some(serializable)).await
}
