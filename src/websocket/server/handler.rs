use crate::global::{AUTHORIZATION, STREAM_SENDERS};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt as _, StreamExt as _};
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

pub(crate) async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket)
}

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    match receiver.next().await {
        Some(Ok(Message::Text(auth))) if auth == AUTHORIZATION.0 => (),
        _ => return,
    }

    let (tx, mut rx) = mpsc::channel(50);

    let uuid = Uuid::new_v4();

    STREAM_SENDERS.lock().await.insert(uuid, tx);

    let mut interval = tokio::time::interval(Duration::from_secs(60));

    let delete = async {
        STREAM_SENDERS.lock().await.remove(&uuid);
    };

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(msg)) if !matches!(msg, Message::Close(_)) => (),
                    _ => {
                        delete.await;
                        break;
                    }
                }
            }
            Some(received) = rx.recv() => {
                let _ = sender.send(Message::Text(received)).await;
            }
            _ = interval.tick() => {
                if sender.send(Message::Ping(Vec::new())).await.is_err() {
                    delete.await;
                    break;
                }
            }
        }
    }
}
