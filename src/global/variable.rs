use super::struct_impl::{MySelf, OnlineFriends};
use dirs_2::home_dir;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

pub(crate) const APP_NAME: &str = "vrcapi-proxy";
pub(crate) const UA: &str = "User-Agent";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "認証情報が不正です。";

pub(crate) static DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
    home_dir()
        .expect("ホームディレクトリの取得に失敗しました。")
        .join(APP_NAME)
});

// Interior mutable
pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: Lazy::new(|| RwLock::new(Vec::new())),
};

pub(crate) static MYSELF: Lazy<MySelf> = Lazy::new(|| MySelf {
    inner: RwLock::new(None),
});

pub(crate) static FAVORITE_FRIENDS: Lazy<RwLock<HashSet<String>>> =
    Lazy::new(|| RwLock::new(HashSet::new()));

pub(crate) static HANDLER: Lazy<RwLock<anyhow::Result<tokio::task::JoinHandle<()>>>> =
    Lazy::new(|| RwLock::new(Err(anyhow::anyhow!("uninitialized"))));

pub(crate) static AUTHORIZATION: Lazy<(&'static str, RwLock<String>)> = Lazy::new(|| {
    let data = crate::general::read_json::<crate::init::Data>("data.json").unwrap();
    (data.auth.leak(), RwLock::new(data.token))
});

pub(crate) static STREAM_SENDERS: Lazy<Mutex<HashMap<Uuid, Sender<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
