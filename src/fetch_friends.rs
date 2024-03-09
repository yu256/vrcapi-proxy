use crate::global::{AUTHORIZATION, FRIENDS, HANDLER};
use crate::user_impl::{Status, User, VecUserExt as _};
use crate::utils::ResponseExt as _;
use crate::websocket::error::WSError::{Disconnected, Other, Unknown};
use crate::{
    api::{fetch_favorite_friends, request},
    websocket::stream::stream,
};
use hyper::Method;

pub(crate) async fn spawn() {
    if let Ok(ref handler) = *HANDLER.read().await {
        if !handler.is_finished() {
            handler.abort();
        }
    }

    *HANDLER.write().await = Ok(tokio::spawn(async move {
        let result: anyhow::Result<()> = async move {
            println!("Trying to connect stream...");

            let token = &AUTHORIZATION.1.read().await;

            let mut friends = request(
                Method::GET,
                "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false",
                token,
            )
            .await?
            .json::<Vec<User>>()
            .await?;

            fetch_favorite_friends(token).await?;

            friends.retain_mut(|friend| {
                let is_online = friend.location != "offline";
                if is_online && let Status::AskMe | Status::Busy = friend.status {
                    friend.undetermined = true;
                }
                is_online
            });
            friends.unsanitize();
            friends.sort();

            FRIENDS.write(|users| *users = friends).await;

            loop {
                match stream().await {
                    Disconnected => (),
                    Other(e) => {
                        eprintln!("{e} ({})", chrono::Local::now())
                    }
                    Unknown(e) => {
                        eprintln!("Unknown Error: {e} ({})", chrono::Local::now());
                        break;
                    }
                    _ => break,
                }
            }
            Ok(())
        }
        .await;

        if let Err(e) = result {
            eprintln!("Error: {e}");
            *HANDLER.write().await = Err(e);
        }
    }));
}
