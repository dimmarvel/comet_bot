use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    chat: Chat,
    date: i64,
    from: User,
    message_id: i64,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chat {
    first_name: String,
    id: i64,
    #[serde(rename = "type")]
    chat_type: String,
    username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
    id: i64,
    is_bot: bool,
    language_code: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Update {
    message: Message,
    update_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Updates {
    ok: bool,
    result: Vec<Update>,
}