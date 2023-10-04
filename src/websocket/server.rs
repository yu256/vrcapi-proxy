use std::time::Duration;
use axum::extract::ws::WebSocket;
use axum::extract::{Query, WebSocketUpgrade, ws::Message};
use axum::response::Response;
use crate::global::{Container, STREAM_MANAGER};

pub(crate) async fn ws_handler(Query(params): Query<Params>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_socket(params, socket))
}

#[derive(serde::Deserialize)]
pub(crate) struct Params {
    pub(crate) auth: String,
    pub(crate) session: String,
}

async fn handle_socket(params: Params, mut ws: WebSocket) {
    STREAM_MANAGER.add(Container::new(params.auth, params.session)).await;
    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        tokio::select! {
            msg = ws.recv() => {
                match msg {
                    Some(msg) => {
                        if let Ok(msg) = msg {
                            if matches!(msg, Message::Close(_)) {
                                break;
                            }
                        } else {
                            break;
                        };
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                if STREAM_MANAGER.inner.write().await.iter().find(|c| c.session == params.session).unwrap().fetch_false() {

                }
                if ws.send(Message::Ping(Vec::new())).await.is_err() {
                    break;
                }
            }
        }
    }
}
