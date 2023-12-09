use anyhow::Result;
use dirs_2::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::LazyLock,
};

pub(crate) static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    home_dir()
        .expect("ホームディレクトリの取得に失敗しました。")
        .join("vrcapi_proxy")
});

pub(crate) fn read_json<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut file = BufReader::new(File::open(DATA_PATH.join(path))?);

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    serde_json::from_str(&content).map_err(From::from)
}

pub(crate) fn write_json<T>(data: &T, name: &str) -> Result<()>
where
    T: Serialize,
{
    let Ok(file) = File::create(DATA_PATH.join(name)) else {
        create_dir_all(&*DATA_PATH)?;
        return write_json(data, name);
    };

    let json = serde_json::to_string(data)?;

    let mut file = BufWriter::new(file);

    file.write_all(json.as_bytes()).map_err(From::from)
}

pub(crate) trait CustomAndThen<T, E> {
    fn and_then2<U, E2, F: FnOnce(T) -> Result<U, E2>>(self, op: F) -> Result<U, E>
    where
        E: std::convert::From<E2>;
}

impl<T, E> CustomAndThen<T, E> for Result<T, E> {
    fn and_then2<U, E2, F: FnOnce(T) -> Result<U, E2>>(self, op: F) -> Result<U, E>
    where
        E: std::convert::From<E2>,
    {
        match self {
            Ok(t) => op(t).map_err(From::from),
            Err(e) => Err(e),
        }
    }
}
