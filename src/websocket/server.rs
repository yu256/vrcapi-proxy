use crate::global::AUTHORIZATION;
use axum::extract::{
    self,
    ws::{Message, WebSocket},
    WebSocketUpgrade,
};
use futures::{SinkExt as _, StreamExt as _};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Query {
    auth: Option<String>,
    auth_token: Option<String>,
}

pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    extract::Query(Query { auth, auth_token }): extract::Query<Query>,
) -> hyper::Response<axum::body::Body> {
    ws.on_upgrade(|stream| websocket(stream, (auth, auth_token)))
}

pub(super) static STREAM_SENDERS: Lazy<Mutex<HashMap<Uuid, Sender<Arc<String>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

async fn websocket(stream: WebSocket, auth: (Option<String>, Option<String>)) {
    let (mut sender, mut receiver) = stream.split();

    if !matches!(auth, (Some(val), _) | (_, Some(val)) if val == AUTHORIZATION.0) {
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
                let _ = sender.send(Message::Text(Arc::unwrap_or_clone(received))).await;
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
