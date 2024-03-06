use crate::{
    api::utils::request_json, global::AUTHORIZATION, init::Data, spawn, split_colon, validate,
};
use anyhow::Result;
use serde_json::json;

pub(crate) async fn api_twofactor(req: String) -> Result<bool> {
    split_colon!(req, [auth, token, r#two_fa_type, two_fa_code]);
    let _ = validate::validate(auth)?;

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{two_fa_type}/verify"),
        token,
        json!({ "code": two_fa_code }),
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

    Ok(true)
}
