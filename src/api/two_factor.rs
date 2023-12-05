use crate::{api::utils::request_json, spawn, split_colon};
use anyhow::{ensure, Result};
use serde_json::json;

pub(crate) async fn api_twofactor(
    req: String,
    credentials: crate::types::Credentials,
) -> Result<String> {
    let mut iter = req.split(':');
    split_colon!(iter, [token, r#type, f, auth]);

    ensure!(auth.chars().count() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    credentials.write().await.1 = token.to_owned();

    spawn(credentials);

    Ok(auth.to_owned())
}
