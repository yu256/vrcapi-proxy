use crate::global::DATA_PATH;
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
};

pub(crate) fn read_json<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = BufReader::new(File::open(DATA_PATH.join(path))?);

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(From::from)
}

pub(crate) fn write_json<T>(serializable: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(DATA_PATH.join(name)) else {
        create_dir_all(&*DATA_PATH)?;
        return write_json(serializable, name);
    };

    let json = serde_json::to_vec(serializable)?;

    let mut file = BufWriter::new(file);

    file.write_all(&json).map_err(From::from)
}
