use super::struct_impl::{MySelf, Friends};
use dirs_2::home_dir;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::RwLock;

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
pub(crate) static FRIENDS: RwLock<Friends> = RwLock::const_new(Friends::new());

pub(crate) static MYSELF: MySelf = MySelf(RwLock::const_new(None));

pub(crate) static FAVORITE_FRIENDS: RwLock<Vec<String>> = RwLock::const_new(Vec::new());

pub(crate) static WS_HANDLER: RwLock<Option<tokio::task::JoinHandle<()>>> = RwLock::const_new(None);

pub(crate) static AUTHORIZATION: Lazy<(&'static str, RwLock<String>)> = Lazy::new(|| {
    let data = crate::json::read_json::<crate::init::Data>("data.json").unwrap();
    (data.auth.leak(), RwLock::new(data.token))
});
