use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub chat: Chat,
    pub date: i64,
    pub from: User,
    pub message_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub first_name: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub first_name: String,
    pub id: i64,
    pub is_bot: bool,
    pub language_code: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub message: Message,
    pub update_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Updates {
    pub ok: bool,
    pub result: Vec<Update>,
}