use super::utils::request;
use crate::split_colon;
use anyhow::Result;

pub(crate) async fn api_friend_accept(
    mut req: std::str::Split<'_, char>,
    token: &str,
) -> Result<()> {
    split_colon!(req, [id]);

    request(
        "PUT",
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"),
        token,
    )?;

    Ok(())
}
