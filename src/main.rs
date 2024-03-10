use crate::init::init;
use anyhow::Result;
use api::*;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::{routing::post, Router};
use fetch_friends::spawn;
use tower_http::cors::CorsLayer;
use websocket::server::handler::ws_handler;

mod api;
mod fetch_friends;
mod general;
mod global;
mod init;
mod notification;
mod unsanitizer;
mod user_impl;
mod validate;
mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    spawn().await;

    let init::Data {
        cors,
        listen,
        auth: _,
        token: _,
    } = general::read_json::<init::Data>("data.json")?;

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route(
            "/reboot",
            post(move |auth: String| async move {
                drop(validate::validate(&auth)?);
                spawn().await;
                Ok(true)
            }),
        )
        .route("/auth", post(api_auth))
        .route("/user", post(api_user))
        .route("/profile", post(api_update_profile))
        .route("/friends", post(api_friends))
        .route("/friends/filtered", post(api_friends_filtered))
        .route("/friend/request", post(api_friend_request))
        .route("/friend/accept", post(api_friend_accept))
        .route("/friend/status", post(api_friend_status))
        .route("/invite/myself", post(api_invite_myself))
        .route("/notifications", post(api_notifications))
        .route("/search/user", post(api_search_user))
        .route("/twofactor", post(api_twofactor))
        .route("/favorites", post(api_add_favorites))
        .route("/favorites/refresh", post(api_re_fetch))
        .route("/group", post(api_group))
        .route("/instance", post(api_instance))
        .route("/world", post(api_world))
        .layer(
            CorsLayer::new()
                .allow_origin(cors.parse::<HeaderValue>()?)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind(&listen).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
