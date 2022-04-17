use mongodb::{
    bson::{doc, Document},
    error::Error as MongoError,
    results::InsertOneResult,
    Collection,
};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashSet;
//use crate::models::mongodb_models::MongoUser;

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

pub async fn validate_token(
    token: &str,
    sessoins_collection: &Collection<Document>,
) -> Result<(Option<String>, String), MongoError> {
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
            message = String::from("token is not valid");
        }
    }
    return Ok((username, message));
}

pub async fn get_subscribers(
    username: &str,
    users_collection: &Collection<Document>,
) -> Result<Option<HashSet<String>>, Box<dyn std::error::Error>> {
    let result = users_collection
        .find_one(doc! {"username": username}, None)
        .await?;
    if let Some(doc) = result {
        let vec: HashSet<String> = doc.get_array("subscribers").unwrap().iter().map(|x| x.as_str().unwrap().to_owned()).collect();
        return Ok(Some(vec));
    }
    Ok(None)
}


