#[derive(Debug)]
pub(crate) enum WSError {
	Unknown,
    Token,
    Other(String),
}