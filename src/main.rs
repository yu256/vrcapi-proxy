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
            if let Ok(friends) = fetch_friends(&data.token).await {
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
        return migrate();
    }

    let conf = CorsConfig {
        url: "http://localhost:3000".to_owned(),
    };
    let data: &[bool; 0] = &[]; // どうせ空なので型は適当

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    std::process::exit(0);
}

// いずれ消す
fn migrate() -> Result<()> {
    #[derive(serde::Deserialize)]
    struct OldData {
        auth: String,
        token: String,
        #[allow(dead_code)]
        askme: bool,
    }
    match read_json::<Vec<OldData>>("data.json") {
        Ok(_) => {
            fn to_new_data(data: OldData) -> Data {
                Data {
                    auth: data.auth,
                    token: data.token,
                }
            }
            match read_json::<Vec<OldData>>("data.json") {
                Ok(data) => {
                    let new_data = data.into_iter().map(to_new_data).collect::<Vec<_>>();
                    write_json(&new_data, "data")?;
                    Ok(())
                }
                Err(_) => panic!("data.json is broken."),
            }
        }
        Err(_) => Ok(()),
    }
}
