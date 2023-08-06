use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/twofactorauth/emailotp/verify";

#[derive(Serialize)]
enum Response {
    Success { verified: bool },
    Error { error: String },
}

#[derive(Deserialize)]
struct Res {
    verified: bool,
}

#[post("/twofactor_email", data = "<req>")]
pub(crate) async fn api_twofactor_email(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(res) => Response::Success {
            verified: res.verified,
        },
        Err(error) => Response::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<Res> {
    let (token, f) = req.split_once(':').context("Unexpected input.")?;
    let res = reqwest::Client::new()
        .get(URL)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
