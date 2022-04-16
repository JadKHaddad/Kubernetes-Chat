use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupModel {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninModel {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AddContactModel {
    pub username: String,
}
