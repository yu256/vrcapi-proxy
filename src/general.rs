use crate::data::DATA_PATH;
use anyhow::Result;
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
