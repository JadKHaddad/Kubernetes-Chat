use serde::Deserialize;

#[derive(Deserialize)]
struct MongoUser {
    email: String,
    username: String,
    signup_time_stamp: u32,
    contacts: Vec<String>,
    subscribers: Vec<String>,
}
