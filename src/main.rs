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
    read_json::<Vec<Data>>("data.json")
        .unwrap()
        .into_iter()
        .for_each(spawn);
    rocket::build().mount("/", route()).attach(cors::Cors)
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

pub(crate) fn spawn(data: Data) {
    tokio::spawn(async move {
        if let Ok(friends) = fetch_friends(&data.token).await {
            FRIENDS.write().await.insert(data.auth.to_owned(), friends);
            loop {
                sleep(std::time::Duration::from_secs(60)).await;
                match fetch_friends(&data.token).await {
                    Ok(f) => {
                        let mut unlocked = FRIENDS.write().await;
                        *unlocked.get_mut(&data.auth).unwrap() = f;
                    }
                    Err(e) => {
                        if e.to_string().contains("Missing Credentials") {
                            break;
                        }
                    }
                }
            }
        }
    });
}
