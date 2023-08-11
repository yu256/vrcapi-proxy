use crate::general::update_data_property;
use anyhow::{Context as _, Result};
use rocket::http::Status;

#[post("/askme", data = "<req>")]
pub(crate) async fn api_toggle(req: &str) -> (Status, String) {
    match toggle(req) {
        Ok(bool) => (Status::Ok, format!("{}に変更しました。", bool)),

        Err(err) => (Status::InternalServerError, err.to_string()),
    }
}

fn toggle(req: &str) -> Result<bool> {
    let (auth, req) = req.split_once(':').context("Unexpected token.")?;

    let bool = if req == "t" { true } else { false };

    update_data_property(auth, |data| {
        data.askme = bool;
    })?;

    Ok(bool)
}
