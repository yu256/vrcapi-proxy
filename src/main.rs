#![feature(lazy_cell)]

use crate::init::{init, Config};
use anyhow::Result;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use fetch_friends::spawn;
use general::read_json;
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

mod api;
mod fetch_friends;
mod general;
mod global;
mod init;
mod macros;
mod unsanitizer;
mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;
    read_json::<HashMap<String, String>>("data.json")?
        .into_iter()
        .for_each(spawn);

    let conf = read_json::<Config>("config.json")?;

    let app = Router::new()
        .route("/auth", post(api::api_auth))
        .route("/user", post(api::api_user))
        .route("/profile", post(api::api_update_profile))
        .route("/friends", post(api::api_friends))
        .route("/friends/filtered", post(api::api_friends_filtered))
        .route("/friend/request", post(api::api_friend_request))
        .route("/friend/accept", post(api::api_friend_accept))
        .route("/friend/status", post(api::api_friend_status))
        .route("/notifications", post(api::api_notifications))
        .route("/search/user", post(api::api_search_user))
        .route("/twofactor", post(api::api_twofactor))
        .route("/favorites", post(api::api_add_favorites))
        .route("/favorites/refresh", post(api::api_re_fetch))
        .route("/group", post(api::api_group))
        .route("/instance", post(api::api_instance))
        .route("/world", post(api::api_world))
        .layer(
            CorsLayer::new()
                .allow_origin(conf.cors.parse::<HeaderValue>()?)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE]),
        );

    axum::Server::bind(&conf.listen.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
