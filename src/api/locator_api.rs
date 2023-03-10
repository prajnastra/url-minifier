use crate::{models::locator_model::Locator, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/locator", data = "<new_locator>")]
pub fn create_locator(
    db: &State<MongoRepo>,
    new_locator: Json<Locator>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Locator {
        id: None,
        url: new_locator.url.to_owned(),
    };

    let locator_detail = db.create_locator(data);

    match locator_detail {
        Ok(locator) => Ok(Json(locator)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/locator/<path>")]
pub fn get_locator(db: &State<MongoRepo>, path: String) -> Result<Json<Locator>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let locator_details = db.get_locator(&id);

    match locator_details {
        Ok(locator) => Ok(Json(locator)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/locator/<path>", data = "<new_locator>")]
pub fn update_locator(
    db: &State<MongoRepo>,
    path: String,
    new_locator: Json<Locator>,
) -> Result<Json<Locator>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let data = Locator {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        url: new_locator.url.to_owned(),
    };
    let update_result = db.update_locator(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_locator_info = db.get_locator(&id);
                return match updated_locator_info {
                    Ok(locator) => Ok(Json(locator)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/locator/<path>")]
pub fn delete_locator(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_locator(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Locator successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/locators")]
pub fn get_all_locators(db: &State<MongoRepo>) -> Result<Json<Vec<Locator>>, Status> {
    let locators = db.get_all_locator();
    match locators {
        Ok(locators) => Ok(Json(locators)),
        Err(_) => Err(Status::InternalServerError),
    }
}
