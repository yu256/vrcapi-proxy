use super::utils::request;
use crate::split_colon;
use anyhow::Result;

pub(crate) async fn api_friend_request(
    mut req: std::str::Split<'_, char>,
    token: &str,
) -> Result<bool> {
    split_colon!(req, [user, method]);

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
        token,
    )
    .map(|_| true)
}
