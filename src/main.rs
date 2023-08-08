#![feature(lazy_cell)]
mod api;
mod cors;
mod data;
mod general;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                api::api_auth,
                api::api_twofactor_email,
                api::api_friends,
                api::api_user,
                api::api_instance,
                api::api_toggle,
                api::api_search_user
            ],
        )
        .attach(cors::CORS)
}
