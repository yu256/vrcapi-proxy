use crate::global::AUTHORIZATION;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt as _, StreamExt as _};
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};
use uuid::Uuid;

pub(crate) async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket)
}

pub(super) static STREAM_SENDERS: Lazy<Mutex<HashMap<Uuid, Sender<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    if !matches!(receiver.next().await, Some(Ok(Message::Text(auth))) if auth == AUTHORIZATION.0) {
        return;
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
                if !matches!(msg, Some(Ok(msg)) if !matches!(msg, Message::Close(_))) {
                    delete.await;
                    break;
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
