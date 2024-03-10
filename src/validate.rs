use crate::global::{AUTHORIZATION, INVALID_AUTH};
use futures::Future;
use tokio::sync::RwLockReadGuard;

pub(crate) fn validate<T>(
    req_auth: T,
) -> anyhow::Result<impl Future<Output = RwLockReadGuard<'static, String>>>
where
    T: AsRef<str>,
{
    let (auth, ref token) = *AUTHORIZATION;
    anyhow::ensure!(req_auth.as_ref() == auth, INVALID_AUTH);
    Ok(token.read())
}
