use anyhow::Result;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::log::private::info;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::data::DATA_PATH;
use crate::general::{open_file, write_json};

pub struct CORS;

#[derive(Serialize, Deserialize)]
struct CorsConfig {
    url: String,
}

impl CorsConfig {
    fn get() -> Result<Self> {
        match open_file(&DATA_PATH.join("config.json")) {
            Ok(mut file) => {
                let mut content = String::new();

                file.read_to_string(&mut content)?;

                Ok(serde_json::from_str(&content)?)
            }
            Err(_) => {
                let conf = CorsConfig {
                    url: "http://localhost:3000".to_owned(),
                };

                write_json(&conf, "config")?;

                info!(
                    "{} を生成しました。",
                    &DATA_PATH.join("config.json").to_string_lossy()
                );

                Ok(conf)
            }
        }
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            CorsConfig::get().unwrap().url,
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
