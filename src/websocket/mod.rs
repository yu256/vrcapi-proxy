mod client;
mod server;

pub(crate) use crate::websocket::client::stream::stream;
pub(crate) use crate::websocket::client::structs::User;
pub(crate) use crate::websocket::client::structs::VecUserExt;
pub(crate) use crate::websocket::server::ws_handler;
