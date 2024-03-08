use super::struct_impl::{MySelf, OnlineFriends};
use dirs_2::home_dir;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::LazyLock,
};
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub(crate) const APP_NAME: &str = "vrcapi_proxy";
pub(crate) const UA: &str = "User-Agent";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "認証情報が不正です。";

pub(crate) static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    home_dir()
        .expect("ホームディレクトリの取得に失敗しました。")
        .join(APP_NAME)
});

// Interior mutable
pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: LazyLock::new(|| RwLock::new(Vec::new())),
};

pub(crate) static MYSELF: LazyLock<MySelf> = LazyLock::new(|| MySelf {
    inner: RwLock::new(None),
});

pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub(crate) static HANDLER: LazyLock<RwLock<anyhow::Result<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| RwLock::new(Err(anyhow::anyhow!("uninitialized"))));

pub(crate) static AUTHORIZATION: LazyLock<(&'static str, RwLock<String>)> = LazyLock::new(|| {
    let data = crate::general::read_json::<crate::init::Data>("data.json").unwrap();
    (data.auth.leak(), RwLock::new(data.token))
});

pub(crate) static STREAM_SENDERS: LazyLock<Mutex<HashMap<Uuid, Sender<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
