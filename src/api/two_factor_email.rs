use crate::data::{Data, DataVecExt as _};
use anyhow::{bail, Context as _, Error, Result};
use serde::Serialize;
use serde_json::{json, to_string as to_json};
use uuid::Uuid;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/twofactorauth/emailotp/verify";

#[derive(Serialize)]
enum Response {
    Success { auth: String },
    Error { error: String },
}

impl From<Error> for Response {
    fn from(error: Error) -> Self {
        Response::Error {
            error: error.to_string(),
        }
    }
}

#[post("/twofactor_email", data = "<req>")]
pub(crate) async fn api_twofactor_email(req: &str) -> String {
    let result: Response = match req.split_once(';') {
        Some((req, auth)) => match fetch(req).await {
            Ok(token) => {
                if let Err(err) = update(token, auth) {
                    return to_json(&Response::from(err)).unwrap();
                }

                Response::Success {
                    auth: auth.to_string(),
                }
            }
            Err(error) => Response::from(error),
        },
        None => match fetch(req).await {
            Ok(token) => {
                let auth = Uuid::new_v4().to_string();

                if let Err(err) = add(token, &auth) {
                    return to_json(&Response::from(err)).unwrap();
                }

                Response::Success { auth }
            }
            Err(error) => Response::from(error),
        },
    };

    to_json(&result).unwrap()
}

async fn fetch(req: &str) -> Result<&str> {
    let (token, f) = req.split_once(':').context("Unexpected input.")?;
    let res = reqwest::Client::new()
        .post(URL)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}

fn update(token: &str, auth: &str) -> Result<()> {
    let mut data: Vec<Data> = Data::get()?;

    if let Some(data) = data.iter_mut().find(|data| data.auth == auth) {
        data.token = token.to_string();
    } else {
        bail!("No matching auth found.");
    }

    data.write()?;

    Ok(())
}

fn add(token: &str, auth: &str) -> Result<()> {
    let new_data = Data {
        auth: auth.to_string(),
        token: token.to_string(),
        askme: false,
    };

    let mut data: Vec<Data> = Data::get()?;

    data.push(new_data);

    data.write()?;

    Ok(())
}
