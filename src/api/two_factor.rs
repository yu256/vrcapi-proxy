use crate::{
    api::utils::request_json, global::AUTHORIZATION, init::Data, spawn, split_colon, validate,
};
use anyhow::{ensure, Result};
use serde_json::json;

pub(crate) async fn api_twofactor(req: String) -> Result<&'static str> {
    split_colon!(req, [token, r#type, f, auth]);
    let auth = validate!(auth);

    ensure!(auth.chars().count() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    let data = {
        let data = crate::general::read_json::<Data>("data.json")?;
        Data {
            listen: data.listen,
            cors: data.cors,
            auth: data.auth,
            token: token.into(),
        }
    };

    crate::general::write_json::<Data>(&data, "data.json")?;

    *AUTHORIZATION.1.write().await = data.token;

    spawn().await;

    Ok(auth)
}
