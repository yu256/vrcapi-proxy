#![feature(lazy_cell)]

use anyhow::Result;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use fetch_friends::spawn;
use general::{read_json, write_json, DATA_PATH};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

mod api;
mod fetch_friends;
mod general;
mod global;
mod macros;
mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;
    read_json::<HashMap<String, String>>("data.json")?
        .into_iter()
        .for_each(spawn);

    let app = Router::new()
        .route("/auth", post(api::api_auth))
        .route("/favorites", post(api::api_add_favorites))
        .route("/favorites/refresh", post(api::api_re_fetch))
        .route("/friend_accept", post(api::api_friend_accept))
        .route("/friend_request", post(api::api_friend_request))
        .route("/friend_status", post(api::api_friend_status))
        .route("/friends", post(api::api_friends))
        .route("/favfriends", post(api::api_friends_filtered))
        .route("/group", post(api::api_group))
        .route("/instance", post(api::api_instance))
        .route("/notifications", post(api::api_notifications))
        .route("/search_user", post(api::api_search_user))
        .route("/twofactor", post(api::api_twofactor))
        .route("/user", post(api::api_user))
        .route("/world", post(api::api_world))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    read_json::<CorsConfig>("config.json")?
                        .url
                        .parse::<HeaderValue>()?,
                )
                .allow_methods([Method::POST]),
        );

    axum::Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn init() -> Result<()> {
    if DATA_PATH.join("data.json").is_file() && DATA_PATH.join("config.json").is_file() {
        return Ok(());
    }

    let conf = CorsConfig {
        url: "http://localhost:3000".to_owned(),
    };
    let data: HashMap<String, String> = HashMap::new();

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    std::process::exit(0);
}

#[derive(Serialize, Deserialize)]
struct CorsConfig {
    pub(crate) url: String,
}
