use crate::websocket::User;
use anyhow::{Context as _, Result};
use std::sync::atomic::AtomicU8;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};
use tokio::sync::RwLock;

pub(crate) const UA: &str = "User-Agent";
pub(crate) const UA_VALUE: &str = "vrcapi-proxy";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "サーバー側の初回fetchに失敗しているか、トークンが無効です。";

pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: LazyLock::new(|| RwLock::new(HashMap::new())),
};
pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) static COLOR: AtomicU8 = AtomicU8::new(1);

pub(crate) struct OnlineFriends {
    inner: LazyLock<RwLock<HashMap<String, Vec<User>>>>,
}

impl OnlineFriends {
    pub(crate) async fn read<T, F>(&self, auth: &str, fun: F) -> Result<T>
    where
        F: FnOnce(&Vec<User>) -> T,
    {
        let friends = self.inner.read().await;
        let friends = friends.get(auth).context(INVALID_AUTH)?;
        Ok(fun(&friends))
    }

    pub(crate) async fn write<F>(&self, auth: &str, fun: F)
    where
        F: FnOnce(&mut Vec<User>),
    {
        let mut friends = self.inner.write().await;
        if let Some(friends) = friends.get_mut(auth) {
            fun(friends)
        }
    }

    pub(crate) async fn insert(&self, key: String, value: Vec<User>) {
        self.inner.write().await.insert(key, value);
    }

    pub(crate) async fn remove(&self, key: &str) {
        self.inner.write().await.remove(key);
    }
}
