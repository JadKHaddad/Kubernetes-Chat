use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupModel {
    pub success: bool,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninModel {
    success: bool,
    message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignoutModel {
    success: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsSignedinModel {
    success: bool,
    username: String,
    message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContactModel {
    success: bool,
    message: String
}
