use crate::unsanitizer::Unsanitizer;
use crate::{
    fetcher::{request, ResponseExt as _},
    global::FRIENDS,
    validate::validate,
};
use anyhow::Result;
use axum::Json;
use hyper::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    instance_id: String,
}

pub(crate) async fn api_instance(
    Json(Query { auth, instance_id }): Json<Query>,
) -> Result<Response> {
    let token = validate(auth)?.await;

    let res = request(
        Method::GET,
        &format!("https://api.vrchat.cloud/api/1/instances/{instance_id}"),
        &token,
    )
    .await?;

    let users = FRIENDS
        .read(|friends| {
            friends
                .iter()
                .filter(|user| user.location == instance_id)
                .map(|user| {
                    (
                        if !user.userIcon.is_empty() {
                            user.userIcon.clone()
                        } else if !user.profilePicOverride.is_empty() {
                            user.profilePicOverride.clone()
                        } else {
                            user.currentAvatarThumbnailImageUrl.clone()
                        },
                        user.displayName.clone(),
                    )
                })
                .collect()
        })
        .await;

    Ok(res.json::<InstanceData>().await?.into_res(users))
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct World {
    name: String,
    description: String,
    thumbnailImageUrl: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct InstanceData {
    ownerId: Option<String>,
    userCount: i32,
    world: World,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    ownerId: Option<String>,
    userCount: i32,
    name: String,
    description: String,
    thumbnailImageUrl: String,
    users: HashMap<String, String>,
}

impl InstanceData {
    fn into_res(self, users: HashMap<String, String>) -> Response {
        Response {
            ownerId: self.ownerId,
            userCount: self.userCount,
            name: self.world.name,
            description: self.world.description.unsanitize(),
            thumbnailImageUrl: self.world.thumbnailImageUrl,
            users,
        }
    }
}
