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
                "token": token,
                "username": username,
            },
            None,
        )
        .await?;
    return Ok(result);
}

pub async fn validate_token(token: &str, sessoins_collection: &Collection<Document>) -> Result<(Option<String>, String), MongoError> {
    let mut username: Option<String> = None;
    let mut message: String = String::from("no token");
    let result = sessoins_collection
        .find_one(doc! {"token": token}, None)
        .await?;
    match result {
        Some(doc) => {
            username = Some(doc.get_str("username").unwrap().to_string());
        }
        None => {
            message = String::from("token is no valid");
        }
    }
    return Ok((username, message));
}
