use mongodb::{bson::Document, Collection};
use serde::Deserialize;

#[derive(Clone)]
pub struct Collections {
    pub users_collection: Collection<Document>,
    pub sessions_collection: Collection<Document>,
}

#[derive(Deserialize)]
pub struct MongoUser {
    pub email: String,
    pub username: String,
    pub signup_time_stamp: u32,
    pub contacts: Vec<String>,
    pub subscribers: Vec<String>,
}


