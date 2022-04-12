pub mod mongodb_models;
pub mod request_models;
pub mod response_models;
pub mod ws_models;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserInfo<'a> {
   pub contacts: Option<&'a std::vec::Vec<mongodb::bson::Bson>>,
}
