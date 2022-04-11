use serde::Deserialize;
use serde::Serialize;
use crate::models::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupModel {
    pub success: bool,
    pub message: String
}

#[derive(Debug, Serialize)]
pub struct SigninModel <'a>{
    pub success: bool,
    pub username: String,
    pub info: UserInfo<'a>,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignoutModel {
    success: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsSignedinModel {
    pub success: bool,
    pub username: Option<String>,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContactModel {
    success: bool,
    message: String
}
