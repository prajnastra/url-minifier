mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::locator_api::{
    create_locator, delete_locator, get_all_locators, get_locator, update_locator,
};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_locator])
        .mount("/", routes![get_locator])
        .mount("/", routes![update_locator])
        .mount("/", routes![delete_locator])
        .mount("/", routes![get_all_locators])
}
