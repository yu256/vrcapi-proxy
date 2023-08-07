#![feature(lazy_cell)]
mod api;
mod data;
mod general;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            hello,
            api::api_auth,
            api::api_twofactor_email,
            api::api_friends,
            api::api_user,
            api::api_instance
        ],
    )
}
