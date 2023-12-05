use super::request;
use anyhow::Result;

pub(crate) async fn api_invite_myself(req: std::str::Split<'_, char>, token: &str) -> Result<bool> {
    let url = req.fold(
        String::from("https://api.vrchat.cloud/api/1/invite/myself/to/"),
        |acc, val| acc + val + ":",
    );
    request("POST", &url[0..url.len() - 1], token).map(|_| true)
}
