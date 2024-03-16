use crate::global::{APP_NAME, COOKIE, UA};
use anyhow::{anyhow, Result};
use bytes::{Buf as _, Bytes};
use http_body_util::{BodyExt as _, Empty};
use hyper::{body::Incoming, Method, Request, Response};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
struct ErrorMessage {
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: String,
    // status_code: u32,
}

static CLIENT: Lazy<Client<HttpsConnector<HttpConnector>, String>> =
    Lazy::new(|| Client::builder(TokioExecutor::new()).build(HttpsConnector::new()));

static GET_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>, Empty<Bytes>>> =
    Lazy::new(|| Client::builder(TokioExecutor::new()).build(HttpsConnector::new()));

pub(super) enum Header<'a> {
    Cookie(&'a str),
    Auth((&'a str, &'a str)),
}

pub(super) async fn make_request(
    method: Method,
    target: &str,
    header: Header<'_>,
    serializable: Option<impl Serialize>,
) -> Result<Response<Incoming>> {
    let builder = Request::builder()
        .method(method)
        .uri(target)
        .header(UA, APP_NAME);

    let builder = match header {
        Header::Cookie(cookie) => builder.header(COOKIE, cookie),
        Header::Auth((header, value)) => builder.header(header, value),
    };

    let response = if let Some(serializable) = serializable {
        CLIENT.request(
            builder
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(&serializable)?)?,
        )
    } else {
        GET_CLIENT.request(builder.body(Empty::new())?)
    }
    .await?;

    if response.status().is_success() {
        Ok(response)
    } else {
        Err(anyhow!(
            "{}",
            response
                .json::<ErrorMessage>()
                .await?
                .error
                .message
                .replace('\"', "")
        ))
    }
}

#[inline]
pub(crate) async fn request(
    method: Method,
    target: &str,
    cookie: &str,
) -> Result<Response<Incoming>> {
    make_request(method, target, Header::Cookie(cookie), None::<()>).await
}

#[inline]
pub(crate) async fn request_json(
    method: Method,
    target: &str,
    cookie: &str,
    serializable: impl Serialize,
) -> Result<Response<Incoming>> {
    make_request(method, target, Header::Cookie(cookie), Some(serializable)).await
}

pub(crate) trait ResponseExt {
    async fn json<T>(self) -> Result<T>
    where
        T: DeserializeOwned;
}

impl ResponseExt for Response<Incoming> {
    async fn json<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let reader = self.collect().await?.aggregate().reader();
        serde_json::from_reader::<_, T>(reader).map_err(From::from)
    }
}
