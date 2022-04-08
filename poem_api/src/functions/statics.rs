use crate::models::UserInfo;
use mongodb::{
    bson::{doc, Document},
    error::Error as MongoError,
    results::InsertOneResult,
    Collection,
};
use rand::{distributions::Alphanumeric, Rng};

pub fn create_token() -> String {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    return token;
}

pub async fn create_session(
    username: &str,
    token: &str,
    sessoins_collection: &Collection<Document>,
) -> Result<InsertOneResult, MongoError> {
    let result = sessoins_collection
        .insert_one(
            doc! {
                "token": username,
                "username": token,
            },
            None,
        )
        .await?;
    return Ok(result);
}
