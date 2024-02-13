use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt as _, StreamExt as _};
use std::{collections::VecDeque, time::Duration};
use uuid::Uuid;

use crate::global::{AUTHORIZATION, STREAM_DEQUE};

pub(crate) async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket)
}

async fn websocket(stream: WebSocket) {
    let (mut sender, mut receiver) = stream.split();

    let Some(Ok(Message::Text(auth))) = receiver.next().await else {
        return;
    };
    if auth != AUTHORIZATION.0 {
        return;
    }

    let uuid = Uuid::new_v4();

    STREAM_DEQUE.lock().await.insert(uuid, VecDeque::new());

    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    let delete = || async {
        STREAM_DEQUE.lock().await.remove(&uuid);
    };

    loop {
        tokio::select! {
            msg = receiver.next() => {
                if let Some(Ok(msg)) = msg && !matches!(msg, Message::Close(_)) {
                } else {
                    delete().await;
                    break;
                };
            }
            _ = interval.tick() => {{
                    let mut locked = STREAM_DEQUE.lock().await;
                    let Some(data) = locked.get_mut(&uuid) else {
                        return;
                    };
                    while let Some(queue) = data.pop_front() {
                        let _ = sender.send(Message::Text(queue)).await;
                    }
                }
                if sender.send(Message::Ping(Vec::new())).await.is_err() {
                    delete().await;
                    break;
                }
            }
        }
    }
}
