#[derive(Debug)]
pub(crate) enum WSError {
    TokenError,
    OtherError(String),
}

#[macro_export]
macro_rules! try_ {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => ok,
            Err(e) => return $crate::websocket::error::WSError::OtherError(e.to_string()),
        }
    };
}
