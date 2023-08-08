use crate::general::update_data_property;
use anyhow::{Context as _, Result};

#[post("/askme", data = "<req>")]
pub(crate) async fn api_toggle(req: &str) -> String {
    match toggle(req) {
        Ok(bool) => format!("{}に変更しました。", bool),
        Err(err) => err.to_string(),
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
