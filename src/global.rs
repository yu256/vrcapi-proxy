use crate::websocket::User;
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

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) static COLOR: AtomicU8 = AtomicU8::new(1);

// todo もっといい実装がありそうな気がするのでいずれなんとかする
pub(crate) struct StreamManager {
    inner: RwLock<Vec<Container>>,
}

pub(crate) struct Container {
    auth: String,
    session: String,
    is_latest: bool,
}

impl Container {
    pub(crate) fn new(auth: impl Into<String>, session: String) -> Self {
        Self {
            auth: auth.into(),
            session,
            is_latest: true,
        }
    }
    pub(crate) fn fetch_false(&mut self) -> bool {
        let current = self.is_latest;
        self.is_latest = true;
        current
    }
}

impl StreamManager {
    fn new() -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
        }
    }
    pub(crate) async fn set_updated(&self, auth: &str) {
        self.inner
            .write()
            .await
            .iter_mut()
            .filter(|c| c.auth == auth)
            .for_each(|c| {
                c.is_latest = false;
            })
    }
    pub(crate) async fn add(&self, container: Container) {
        self.inner.write().await.push(container);
    }
    pub(crate) async fn remove(&self, session: &str) {
        let mut write = self.inner.write().await;
        if let Some(index) = write.iter().position(|x| x.session == session) {
            write.remove(index);
        }
    }
}

pub(crate) static STREAM_MANAGER: LazyLock<StreamManager> = LazyLock::new(|| StreamManager::new());
