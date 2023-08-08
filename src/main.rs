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
                api::api_search_user,
                api::api_friend_request,
                api::api_del_friend_request,
                api::api_friend_status,
                api::api_notifications
            ],
        )
        .attach(cors::CORS)
}
