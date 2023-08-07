use crate::data::{Data, DataVecExt as _};
use anyhow::{bail, Context, Result};

#[post("/askme", data = "<req>")]
pub(crate) async fn api_toggle(req: &str) -> String {
    match toggle(req) {
        Ok(bool) => format!("{}に変更しました。", bool),
        Err(err) => err.to_string(),
    }
}

fn toggle(req: &str) -> Result<bool> {
    let (auth, req) = req.split_once(':').context("Unexpected token.")?;

    let mut data = Data::get()?;

    let bool = if req == "t" { true } else { false };

    if let Some(data) = data.iter_mut().find(|data| data.auth == auth) {
        data.askme = bool;
    } else {
        bail!("No matching auth found.");
    }

    data.write()?;

    Ok(bool)
}
