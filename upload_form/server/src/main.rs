mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::post_api::{create_post, get_post, update_post, delete_post, get_all_posts};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_post])
        .mount("/", routes![get_post])
        .mount("/", routes![update_post])
        .mount("/", routes![delete_post])
        .mount("/", routes![get_all_posts])
}