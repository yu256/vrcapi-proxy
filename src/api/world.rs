use super::utils::{request, ResponseExt as _};
use crate::unsanitizer::Unsanitizer;
use crate::validate::validate;
use anyhow::Result;
use axum::Json;
use hyper::Method;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    world_id: String,
}

pub(crate) async fn api_world(Json(Query { auth, world_id }): Json<Query>) -> Result<World> {
    let token = validate(auth)?.await;

    request(
        Method::GET,
        &format!("https://api.vrchat.cloud/api/1/worlds/{world_id}"),
        &token,
    )
    .await?
    .json::<World>()
    .await
    .map(|mut world| {
        world.modify();
        world
    })
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct World {
    authorId: String,
    // authorName: String,
    capacity: u32,
    created_at: String,
    description: String,
    favorites: u32,
    featured: bool,
    heat: u32,
    // id: String,
    imageUrl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<Vec<Option<Vec<serde_json::Value>>>>,
    labsPublicationDate: String,
    name: String,
    namespace: String,
    // occupants: u32,
    organization: String,
    popularity: u32,
    // previewYoutubeId: Option<String>,
    privateOccupants: u32,
    publicOccupants: u32,
    publicationDate: String,
    // releaseStatus: String,
    tags: Vec<String>,
    thumbnailImageUrl: String,
    // unityPackages: Vec<UnityPackage>,
    updated_at: String,
    // version: u32,
    visits: u32,
}

impl World {
    fn modify(&mut self) {
        self.tags.retain_mut(|tag| {
            let is_tag = tag.starts_with("author_tag");
            if is_tag {
                tag.replace_range(..11, "");
            }
            is_tag
        });
        self.description = self.description.unsanitize();
    }
}
