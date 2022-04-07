use serde::Deserialize;
use mongodb::{
    bson::Document,
    Collection,
};

#[derive(Clone)]
pub struct Collections {
    pub users_collection: Collection<Document>,
    pub sessions_collection: Collection<Document>
}

#[derive(Deserialize)]
struct MongoUser {
    email: String,
    username: String,
    signup_time_stamp: u32,
    contacts: Vec<String>,
    subscribers: Vec<String>,
}
