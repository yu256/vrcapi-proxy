use super::request;
use anyhow::Result;

const URL: &str = "https://api.vrchat.cloud/api/1/invite/myself/to/";

pub(crate) async fn api_invite_myself(req: std::str::Split<'_, char>, token: &str) -> Result<bool> {
    let instance_id: String = req.collect();
    let url = format!("{}{}", URL, instance_id);
    request("POST", &url, token).map(|_| true)
}
