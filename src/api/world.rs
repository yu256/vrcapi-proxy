use crate::{
    consts::{COOKIE, INVALID_INPUT, UA, UA_VALUE},
    general::find_matched_data,
    CLIENT,
};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct World {
    authorId: String,
    authorName: String,
    capacity: u32,
    created_at: String,
    description: String,
    favorites: u32,
    featured: bool,
    heat: u32,
    id: String,
    imageUrl: String,
    // instances: Option<Vec<Instance>>,
    labsPublicationDate: String,
    name: String,
    namespace: String,
    occupants: u32,
    organization: String,
    popularity: u32,
    // previewYoutubeId: Option<String>,
    privateOccupants: u32,
    publicOccupants: u32,
    publicationDate: String,
    releaseStatus: String,
    tags: Vec<String>,
    thumbnailImageUrl: String,
    // unityPackages: Vec<UnityPackage>,
    updated_at: String,
    version: u32,
    visits: u32,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(World),
    Error(String),
}

const URL: &str = "https://api.vrchat.cloud/api/1/worlds/";

#[post("/world", data = "<req>")]
pub(crate) async fn api_world(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(Response::Success(status))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<World> {
    let (auth, world) = req.split_once(':').context(INVALID_INPUT)?;

    let matched = find_matched_data(auth)?;

    let res = CLIENT
        .get(&format!("{URL}{world}"))
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .send()
        .await?;

    if res.status().is_success() {
        let status: World = res.json().await?;
        Ok(status)
    } else {
        bail!("{}", res.status())
    }
}
