use crate::consts::{COOKIE, UA, UA_VALUE};
use anyhow::{Context as _, Result};
use reqwest::Response;
use std::sync::LazyLock;

const INVALID_INPUT: &str = "Invalid input format.";

pub(crate) static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(crate) async fn request(
    method: reqwest::Method,
    target: &str,
    cookie: &str,
) -> Result<Response, reqwest::Error> {
    CLIENT
        .request(method, target)
        .header(UA, UA_VALUE)
        .header(COOKIE, cookie)
        .send()
        .await
}

pub(crate) trait StrExt {
    fn split_colon(&self) -> Result<(&str, &str)>;
}

impl StrExt for str {
    fn split_colon(&self) -> Result<(&str, &str)> {
        Ok(self.split_once(':').context(INVALID_INPUT)?)
    }
}
