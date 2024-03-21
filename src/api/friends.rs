use crate::global::{FAVORITE_FRIENDS, USERS, WS_HANDLER};
use crate::user::User;
use crate::validate::validate;
use anyhow::{ensure, Result};
use serde::Serialize;

async fn validate_(auth: &str) -> Result<()> {
    drop(validate(auth)?);

    ensure!(
        WS_HANDLER.read().await.is_some(),
        "WebSocketに接続されていません。"
    );
    Ok(())
}

pub(crate) async fn api_friends(auth: String) -> Result<ResFriend> {
    validate_(&auth).await?;

    let (public, private) = USERS
        .read()
        .await
        .online
        .iter()
        .cloned()
        .partition(|friend| friend.location.as_ref().is_some_and(|l| l != "private"));

    Ok(ResFriend { public, private })
}

pub(crate) async fn api_friends_filtered(auth: String) -> Result<ResFriend> {
    let favorites = FAVORITE_FRIENDS.read().await;
    api_friends(auth).await.map(|mut friends| {
        let fun = |friend: &User| favorites.contains(&friend.id);
        friends.private.retain(fun);
        friends.public.retain(fun);
        friends
    })
}

pub(crate) async fn api_friends_all(auth: String) -> Result<String> {
    validate_(&auth).await?;
    serde_json::to_string(&*USERS.read().await).map_err(From::from)
}

#[derive(Serialize)]
pub(crate) struct ResFriend {
    public: Vec<User>,
    private: Vec<User>,
}
