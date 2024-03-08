use super::utils::{make_request, Header};
use anyhow::{Context as _, Result};
use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Method;
use trie_match::trie_match;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct TwoFactor {
    requiresTwoFactorAuth: Vec<String>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
pub(crate) struct Response {
    token: String,
    auth_type: &'static str,
}

pub(crate) async fn api_auth(Json(Query { username, password }): Json<Query>) -> Result<Response> {
    let res = make_request(
        Method::GET,
        URL,
        Header::Auth((
            "Authorization",
            &format!(
                "Basic {}",
                general_purpose::STANDARD_NO_PAD.encode(username + ":" + &password)
            ),
        )),
        None::<()>,
    )
    .await?;

    let token = String::from("auth=")
        + res
            .headers()
            .get("set-cookie")
            .and_then(|h| h.to_str().ok())
            .and_then(|c| c.split(';').next())
            .and_then(|c| c.split('=').nth(1))
            .context("invalid cookie found.")?;

    let auth_type = res
        .json::<TwoFactor>()
        .await?
        .requiresTwoFactorAuth
        .into_iter()
        .find_map(|auth| {
            trie_match! {
                match auth.as_str() {
                    "emailOtp" => Some("emailotp"),
                    "totp" => Some("totp"),
                    _ => None,
                }
            }
        })
        .unwrap_or("otp");

    Ok(Response { token, auth_type })
}
