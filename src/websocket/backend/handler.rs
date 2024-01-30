use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt as _, StreamExt as _};
use std::time::Duration;

pub(crate) async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket)
}

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    let Some(Ok(Message::Text(_))) = receiver.next().await else {
        return;
    };

    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
            msg = receiver.next() => {
                if let Some(Ok(msg)) = msg && !matches!(msg, Message::Close(_)) {
                } else {
                    break;
                };
            }
            _ = interval.tick() => {
                if sender.send(Message::Ping(Vec::new())).await.is_err() {
                    break;
                }
            }
        }
    }
}
