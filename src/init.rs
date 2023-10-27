use crate::general::{write_json, DATA_PATH};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) listen: String,
    pub(crate) cors: String,
}

pub(crate) fn init() -> Result<()> {
    if DATA_PATH.join("data.json").is_file() && DATA_PATH.join("config.json").is_file() {
        return Ok(());
    }

    let conf = Config {
        listen: "0.0.0.0:8000".into(),
        cors: "http://localhost:3000".into(),
    };
    let data: HashMap<String, String> = HashMap::new();

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    println!(
        "{}にコンフィグファイルを生成しました。",
        DATA_PATH.display()
    );

    std::process::exit(0);
}
