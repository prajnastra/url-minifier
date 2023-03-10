use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::locator_model::Locator;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Locator>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("locatorDB");
        let col: Collection<Locator> = db.collection("Locator");

        MongoRepo { col }
    }

    pub fn create_locator(&self, new_user: Locator) -> Result<InsertOneResult, Error> {
        let new_doc = Locator {
            id: None,
            url: new_user.url,
        };
        let locator = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating locator...");

        Ok(locator)
    }

    pub fn get_locator(&self, id: &String) -> Result<Locator, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let locator_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting locator's details");
        Ok(locator_detail.unwrap())
    }

    pub fn update_locator(&self, id: &String, new_locator: Locator) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let new_doc = doc! {
            "$set": {
                "id": new_locator.id,
                "url": new_locator.url,
            }
        };

        let update_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating locator");
        Ok(update_doc)
    }

    pub fn delete_locator(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let locator_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting locator");
        Ok(locator_detail)
    }

    pub fn get_all_locator(&self) -> Result<Vec<Locator>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let locators = cursors.map(|doc| doc.unwrap()).collect();
        Ok(locators)
    }
}
