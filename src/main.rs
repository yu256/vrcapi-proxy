#![feature(lazy_cell)]

use anyhow::Result;
use api::{fetch_friends, route, FRIENDS};
use cors::CorsConfig;
use data::Data;
use general::{get_data, write_json, DATA_PATH};
use rocket::tokio::runtime::Runtime;
use stream::stream;

mod api;
mod consts;
mod cors;
mod data;
mod general;
mod stream;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    init().unwrap();
    get_data::<Vec<Data>>("data.json")
        .unwrap()
        .into_iter()
        .for_each(|data| {
            Runtime::new().unwrap().spawn(async move {
                let friends = fetch_friends(&data.token).await;
                if let Ok(friends) = friends {
                    FRIENDS.lock().await.insert(data.auth.to_owned(), friends);
                    loop {
                        if let Err(e) = stream(data.clone()).await {
                            println!("{e}"); // todo 認証エラーの場合break
                        }
                    }
                }
            });
        });
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
