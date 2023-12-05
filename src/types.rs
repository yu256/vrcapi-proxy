use tokio::sync::RwLock;

pub(crate) type Credentials = &'static RwLock<(&'static str, String)>;
