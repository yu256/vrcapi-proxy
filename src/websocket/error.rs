use tokio_tungstenite::tungstenite;

pub(super) enum WSError {
    Disconnected,
    Token,
    Unknown(String),
    IoErr(tungstenite::error::Error),
}