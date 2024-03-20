use crate::{
    fetcher::{request, ResponseExt as _},
    user::User,
};
use hyper::Method;

pub(super) async fn fetch_all_friends(token: &str, is_offline: bool) -> anyhow::Result<Vec<User>> {
    let mut offset = 0u16;
    let mut friends = Vec::new();

    loop {
        let friends_ = fetch_friends(token, is_offline, offset).await?;
        if friends_.is_empty() {
            break;
        }
        friends.extend(friends_);
        offset += 50;
    }

    friends.iter_mut().for_each(User::unsanitize);

    Ok(friends)
}

async fn fetch_friends(token: &str, is_offline: bool, offset: u16) -> anyhow::Result<Vec<User>> {
    request(
        Method::GET,
        &format!("https://api.vrchat.cloud/api/1/auth/user/friends?offline={is_offline}&n=50&offset={offset}"),
        token,
    )
    .await?
    .json()
    .await
}
