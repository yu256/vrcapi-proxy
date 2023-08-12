use anyhow::Result;
use dirs_2::home_dir;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::general::{open_file, write_json};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub auth: String,
    pub token: String,
    pub askme: bool,
}

impl Data {
    pub(crate) fn get() -> Result<Vec<Self>> {
        let mut file = open_file(&DATA_PATH.join("data.json"))?;

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        Ok(serde_json::from_str(&content)?)
    }
}

pub(crate) trait DataVecExt {
    fn write(&self) -> Result<()>;
}

impl DataVecExt for Vec<Data> {
    fn write(&self) -> Result<()> {
        write_json(self, "data")?;
        Ok(())
    }
}

pub(crate) static DATA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| home_dir().unwrap().join("vrcapi_proxy"));
