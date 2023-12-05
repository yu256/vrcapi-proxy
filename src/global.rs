use crate::websocket::User;
use std::sync::atomic::AtomicU8;
use std::{collections::HashSet, sync::LazyLock};
use tokio::sync::RwLock;

pub(crate) const UA: &str = "User-Agent";
pub(crate) const UA_VALUE: &str = "vrcapi-proxy";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "サーバー側の初回fetchに失敗しているか、トークンが無効です。";
pub(crate) const INVALID_REQUEST: &str = "リクエストが不正です。";

pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: LazyLock::new(|| RwLock::new(Vec::new())),
};

pub(crate) static USERS: LazyLock<Users> = LazyLock::new(|| Users {
    inner: RwLock::new(None),
});

pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub(crate) static COLOR: AtomicU8 = AtomicU8::new(1);

pub(crate) struct OnlineFriends {
    inner: LazyLock<RwLock<Vec<User>>>,
}

impl OnlineFriends {
    pub(crate) async fn read<T, F>(&self, fun: F) -> T
    where
        F: FnOnce(&Vec<User>) -> T,
    {
        let friends = self.inner.read().await;
        fun(&friends)
    }

    pub(crate) async fn write<F>(&self, fun: F)
    where
        F: FnOnce(&mut Vec<User>),
    {
        let mut friends = self.inner.write().await;
        fun(&mut friends)
    }
}

pub(crate) struct Users {
    inner: RwLock<Option<User>>,
}

impl Users {
    pub(crate) async fn insert(&self, user: User) {
        *self.inner.write().await = Some(user);
    }
    pub(crate) async fn read(&self) -> Option<User> {
        self.inner.read().await.clone()
    }
    pub(crate) async fn write(&self, fun: impl FnOnce(&mut User)) {
        if let Some(ref mut user) = *self.inner.write().await {
            fun(user)
        }
    }
}
