use crate::data::{Data, DataVecExt as _, DATA_PATH};
use anyhow::{bail, Context as _, Result};
use serde::Serialize;
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

pub fn open_file(path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(&DATA_PATH.join(format!("{}.json", name))) else {
        fs::create_dir_all(&*DATA_PATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(&data)?;

    let mut file = BufWriter::new(file);
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn find_matched_data(auth: &str) -> Result<Data> {
    let data = Data::get()?;

    let matched: Data = data
        .into_iter()
        .find(|d| d.is_match(auth))
        .context("Failed to auth.")?;

    Ok(matched)
}

pub fn update_data_property<T>(auth: &str, updater: impl Fn(&mut Data) -> T) -> Result<()> {
    let mut data: Vec<Data> = Data::get()?;

    if let Some(data) = data.iter_mut().find(|data| data.auth == auth) {
        updater(data);
    } else {
        bail!("No matching auth found.");
    }

    data.write()?;

    Ok(())
}
