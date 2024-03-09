#[derive(Debug)]
pub(crate) enum WSError {
    Disconnected,
    Token,
    Unknown(String),
    Other(String),
}
