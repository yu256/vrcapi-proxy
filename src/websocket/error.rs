pub(super) enum WSError {
    Disconnected,
    Token,
    Unknown(String),
    Other(String),
}
