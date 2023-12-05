#![feature(lazy_cell)]

use crate::init::{init, Data};
use anyhow::Result;
use api::*;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use fetch_friends::spawn;
use general::read_json;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

mod api;
mod fetch_friends;
mod general;
mod global;
mod init;
mod macros;
mod types;
mod unsanitizer;
mod websocket;

macro_rules! with_credentials {
    ($func:ident, $credentials:expr, None) => {
        move |req: String| async move {
            if req.split(':').next() == Some($credentials.read().await.0) {
                $func().await
            } else {
                Err(anyhow::anyhow!($crate::global::INVALID_AUTH))
            }
        }
    };
    ($func:ident, $credentials:expr) => {
        move |req: String| async move {
            let (auth, ref token) = *$credentials.read().await;
            let mut iter = req.split(':');
            if iter.next() == Some(auth) {
                $func(iter, token).await
            } else {
                Err(anyhow::anyhow!($crate::global::INVALID_AUTH))
            }
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    let data = read_json::<Data>("data.json")?;

    let credentials: (&'static _, _) = (data.auth.leak(), data.token);

    let credentials: &'static RwLock<(&str, String)> =
        Box::leak(Box::new(RwLock::new(credentials)));

    spawn(credentials);

    let app = Router::new()
        .route("/auth", post(api::api_auth))
        .route("/user", post(with_credentials!(api_user, credentials)))
        .route(
            "/profile",
            post(move |req| api_update_profile(req, credentials)),
        )
        .route(
            "/friends",
            post(with_credentials!(api_friends, credentials, None)),
        )
        .route(
            "/friends/filtered",
            post(with_credentials!(api_friends_filtered, credentials, None)),
        )
        .route(
            "/friend/request",
            post(with_credentials!(api_friend_request, credentials)),
        )
        .route(
            "/friend/accept",
            post(with_credentials!(api_friend_accept, credentials)),
        )
        .route(
            "/friend/status",
            post(with_credentials!(api_friend_status, credentials)),
        )
        .route(
            "/invite/myself",
            post(with_credentials!(api_invite_myself, credentials)),
        )
        .route(
            "/notifications",
            post(with_credentials!(api_notifications, credentials)),
        )
        .route(
            "/search/user",
            post(with_credentials!(api_search_user, credentials)),
        )
        .route(
            "/twofactor",
            post(move |req| api_twofactor(req, credentials)),
        )
        .route(
            "/favorites",
            post(with_credentials!(api_add_favorites, credentials)),
        )
        .route(
            "/favorites/refresh",
            post(with_credentials!(api_re_fetch, credentials)),
        )
        .route("/group", post(with_credentials!(api_group, credentials)))
        .route(
            "/instance",
            post(with_credentials!(api_instance, credentials)),
        )
        .route("/world", post(with_credentials!(api_world, credentials)))
        .layer(
            CorsLayer::new()
                .allow_origin(data.cors.parse::<HeaderValue>()?)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE]),
        );

    axum::Server::bind(&data.listen.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
