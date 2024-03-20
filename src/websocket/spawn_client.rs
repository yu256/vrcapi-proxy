use crate::global::WS_HANDLER;
use crate::websocket::client::stream;
use crate::websocket::error::WSError::{Disconnected, IoErr, Unknown};
use std::time::Duration;

pub(crate) async fn spawn_ws_client() {
    if let Some(ref handler) = *WS_HANDLER.read().await {
        if !handler.is_finished() {
            handler.abort();
        }
    }

    *WS_HANDLER.write().await = Some(tokio::spawn(async move {
        let mut io_err_cnt = 0u8;

        loop {
            match stream().await {
                Disconnected => {
                    io_err_cnt = 0;
                }
                Unknown(e) => {
                    eprintln!("Unknown Error: {e}");
                    break;
                }
                IoErr(e) => {
                    io_err_cnt += 1;

                    eprintln!("{e}\nretry: {io_err_cnt}/20");

                    match io_err_cnt {
                        1 => (),
                        20 => break,
                        _ => tokio::time::sleep(Duration::from_secs(10)).await,
                    }
                }
                _ => break,
            }
        }
        *WS_HANDLER.write().await = None;
    }));
}
