use crate::global::{AUTHORIZATION, INVALID_AUTH};
use futures::Future;
use tokio::sync::RwLockReadGuard;

pub(crate) fn validate(
    req_auth: &str,
) -> anyhow::Result<impl Future<Output = RwLockReadGuard<'_, String>>> {
    let (auth, ref token) = *AUTHORIZATION;
    anyhow::ensure!(req_auth == auth, INVALID_AUTH);
    Ok(token.read())
}
