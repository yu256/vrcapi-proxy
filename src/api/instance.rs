use anyhow::{bail, Context as _, Result};
use serde::{Deserialize, Serialize};

use crate::general::find_matched_data;

const URL: &str = "https://api.vrchat.cloud/api/1/instances/";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct World {
    name: String,
    description: String,
    thumbnailImageUrl: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct InstanceData {
    ownerId: Option<String>,
    userCount: i32,
    world: World,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct ResponseInstance {
    ownerId: Option<String>,
    userCount: i32,
    name: String,
    description: String,
    thumbnailImageUrl: String,
}

impl InstanceData {
    fn to_res(self) -> ResponseInstance {
        ResponseInstance {
            ownerId: self.ownerId,
            userCount: self.userCount,
            name: self.world.name,
            description: self.world.description,
            thumbnailImageUrl: self.world.thumbnailImageUrl,
        }
    }
}

#[derive(Serialize)]
enum Response {
    Success(ResponseInstance),
    Error(String),
}

#[post("/instance", data = "<req>")]
pub(crate) async fn api_instance(req: &str) -> String {
    let result = match fetch(req).await {
        Ok(data) => Response::Success(data.to_res()),
        Err(error) => Response::Error(error.to_string()),
    };

    serde_json::to_string(&result).unwrap()
}

async fn fetch(req: &str) -> Result<InstanceData> {
    let (auth, instance) = req.split_once(':').context("Unexpected input.")?;

    let matched = find_matched_data(auth)?;

    let res = reqwest::Client::new()
        .get(&format!("{}{}", URL, instance))
        .header("User-Agent", "vrc-rs")
        .header("Cookie", &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let instance_data: InstanceData = res.json().await?;
        Ok(instance_data)
    } else {
        bail!("Error: status code: {}", res.status())
    }
}
