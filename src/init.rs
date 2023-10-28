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
    let is_data_exist = DATA_PATH.join("data.json").is_file();
    let is_config_exist = DATA_PATH.join("config.json").is_file();

    if is_config_exist && is_data_exist {
        return Ok(());
    }

    if !is_data_exist {
        let data: HashMap<String, String> = HashMap::new();
        write_json(&data, "data")?;
        return Ok(());
    }

    if !is_config_exist {
        let conf = Config {
            listen: "0.0.0.0:8000".into(),
            cors: "http://localhost:3000".into(),
        };
        write_json(&conf, "config")?;
    }

    println!("{}にjsonを生成しました。", DATA_PATH.display());

    std::process::exit(0);
}
