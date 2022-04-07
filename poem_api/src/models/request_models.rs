use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupModel {
    pub email: String,
    pub password: String,
    pub username: String,
}
