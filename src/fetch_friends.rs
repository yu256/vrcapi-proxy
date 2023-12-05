use crate::global::{COLOR, FRIENDS};
use crate::websocket::structs::{Status, VecUserExt as _};
use crate::websocket::User;
use crate::{
    api::{fetch_favorite_friends, request},
    websocket::stream::stream,
};
use std::sync::atomic::Ordering;

pub(crate) fn fetch_friends(token: &str) -> anyhow::Result<Vec<User>> {
    request(
        "GET",
        "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false",
        token,
    )?
    .into_json()
    .map_err(From::from)
}

pub(crate) fn spawn(data: crate::types::Credentials) {
    tokio::spawn(async move {
        let color = COLOR.fetch_add(1, Ordering::Relaxed);

        println!(
            "\x1b[38;5;{}mTrying to connect stream... ({})\x1b[m",
            color,
            data.read().await.0
        );

        match fetch_friends(&data.read().await.1) {
            Ok(mut friends) => {
                let _ = fetch_favorite_friends(&data.read().await.1).await;

                friends.retain_mut(|friend| {
                    if friend.location == "offline" {
                        false
                    } else {
                        if let Status::AskMe | Status::Busy = friend.status {
                            friend.undetermined = true;
                        }
                        true
                    }
                });

                friends.unsanitize();
                friends.sort();

                FRIENDS.write(|users| *users = friends).await;

                loop {
                    if stream(data).await.is_ok() {
                        println!("\x1b[38;5;{color}mトークンが失効しました。");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("\x1b[38;5;{}mError: {}\x1b[m", color, e);
            }
        }
    });
}
