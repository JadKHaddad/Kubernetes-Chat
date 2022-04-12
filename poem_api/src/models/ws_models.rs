use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct WSIncomming<'a> {
    pub username_to: Option<&'a str>,
    pub r#type: &'a str,
    pub text_content: Option<&'a str>,
    pub message_id: Option<&'a str>,
    pub usernames: Option<std::vec::Vec<&'a str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<'a> {
    pub event_type: &'a str,
    pub event_content: EventContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventContent<'a> {
    pub username_target: &'a str,
    pub message_content: MessageContent<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageContent<'a> {
    pub r#type: &'a str,
    pub text_content: &'a str,
    pub date: &'a str,
    pub id: &'a str,
    pub received: bool,
}
