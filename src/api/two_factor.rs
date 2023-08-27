use super::utils::{update_data_property, StrExt as _, CLIENT};
use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    data::{Data, DataVecExt as _},
    general::read_json,
    spawn,
};
use anyhow::{bail, Error, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize)]
pub(crate) enum Res {
    Success(String),
    Error(String),
}

impl From<Error> for Res {
    fn from(error: Error) -> Self {
        Res::Error(error.to_string())
    }
}

#[post("/twofactor", data = "<req>")]
pub(crate) async fn api_twofactor(req: &str) -> (Status, Json<Res>) {
    match req.split_once(';') {
        Some((req, auth)) => match fetch(req).await {
            Ok(token) => {
                if let Err(err) = update(token, auth) {
                    return (Status::InternalServerError, Json(Res::from(err)));
                }

                (Status::Ok, Json(Res::Success(auth.to_string())))
            }

            Err(err) => (Status::InternalServerError, Json(Res::from(err))),
        },

        None => match fetch(req).await {
            Ok(token) => {
                let auth = Uuid::new_v4().to_string();

                if let Err(err) = add(token, &auth) {
                    return (Status::InternalServerError, Json(Res::from(err)));
                }

                (Status::Ok, Json(Res::Success(auth)))
            }

            Err(err) => (Status::InternalServerError, Json(Res::from(err))),
        },
    }
}

async fn fetch(req: &str) -> Result<&str> {
    let (token, rest) = req.split_colon()?;
    let (r#type, f) = rest.split_colon()?;

    let res = CLIENT
        .post(format!(
            "https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"
        ))
        .header(UA, UA_VALUE)
        .header(COOKIE, token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        bail!("{}", res.text().await?)
    }
}

fn update(token: &str, auth: &str) -> Result<()> {
    let data = update_data_property(auth, |data| {
        data.token = token.to_string();
    })?;

    spawn(data);

    Ok(())
}

fn add(token: &str, auth: &str) -> Result<()> {
    let new_data = Data {
        auth: auth.to_string(),
        token: token.to_string(),
    };

    let mut data: Vec<Data> = read_json("data.json")?;

    data.push(new_data);

    data.write()?;

    spawn(unsafe {
        data.into_iter()
            .find(|data| data.auth == auth)
            .unwrap_unchecked()
    });

    Ok(())
}
