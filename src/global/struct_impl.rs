use crate::user::User;
use tokio::sync::RwLock;

pub(crate) struct Friends {
    pub online: Vec<User>,
    pub web: Vec<User>,
    pub offline: Vec<User>,
}

impl Friends {
    pub(super) const fn new() -> Self {
        Self {
            online: Vec::new(),
            web: Vec::new(),
            offline: Vec::new(),
        }
    }
}

pub(crate) struct MySelf(pub(crate) RwLock<Option<User>>);

impl MySelf {
    pub(crate) async fn insert(&self, user: User) {
        *self.0.write().await = Some(user);
    }
    pub(crate) async fn read(&self) -> Option<User> {
        self.0.read().await.clone()
    }
    pub(crate) async fn write(&self, fun: impl FnOnce(&mut User)) {
        if let Some(ref mut user) = *self.0.write().await {
            fun(user)
        }
    }
}
