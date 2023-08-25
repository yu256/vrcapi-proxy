#![feature(lazy_cell)]

use anyhow::Result;
use api::{fetch_friends, route, FRIENDS};
use cors::CorsConfig;
use data::Data;
use general::{read_json, write_json, DATA_PATH};
use rocket::tokio::{self, time::sleep};
// use stream::stream;

mod api;
mod consts;
mod cors;
mod data;
mod general;
// mod stream;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    init().unwrap();
    for data in read_json::<Vec<Data>>("data.json").unwrap() {
        tokio::spawn(async move {
            let friends = fetch_friends(&data.token).await;
            if let Ok(friends) = friends {
                FRIENDS.write().await.insert(data.auth.to_owned(), friends);
                loop {
                    // if let Err(e) = stream(data.clone()).await {
                    //     println!("{e}"); // todo 認証エラーの場合break
                    //     if !e.to_string().contains("Connection reset without closing handshake") {
                    //         panic!();
                    //     }
                    // }
                    sleep(std::time::Duration::from_secs(60)).await;
                    if let Ok(f) = fetch_friends(&data.token).await {
                        let mut unlocked = FRIENDS.write().await;
                        let friends = unlocked.get_mut(&data.auth).unwrap();
                        *friends = f;
                    }
                }
            }
        });
    }

    rocket::build().mount("/", route()).attach(cors::CORS)
}

fn init() -> Result<()> {
    if DATA_PATH.is_dir() {
        return Ok(());
    }

    let conf = CorsConfig {
        url: "http://localhost:3000".to_owned(),
    };
    let data: &[bool; 0] = &[]; // どうせ空なので型は適当

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    std::process::exit(0);
}
