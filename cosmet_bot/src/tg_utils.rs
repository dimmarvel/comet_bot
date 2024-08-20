use serde_json::Value;
use crate::tg_objects::Message;

#[derive(Debug)]
pub enum MsgType {
    GetMe,
    GetUpdates,
    SendMessage,
}

pub enum CommandType {
    Hello,
}

pub fn msg_type_to_str(t: &MsgType) -> &'static str 
{
    match t {
        MsgType::GetMe => "getMe",
        MsgType::GetUpdates => "getUpdates",
        MsgType::SendMessage => "sendMessage",
    }
}

pub fn command_type_to_str(t: &CommandType) -> &'static str 
{
    match t {
        CommandType::Hello => "hello",
    }
}

pub fn command_str_to_type(t: &str) -> Option<CommandType> {
    match t.to_lowercase().as_str() {
        "hello" => Some(CommandType::Hello),
        _ => None,
    }
}

pub fn find_chat_id(json: &Value) -> Option<i64> {
    match json {
        Value::Object(map) => {
            if let Some(Value::Object(chat)) = map.get("chat") {
                if let Some(Value::Number(id)) = chat.get("id") {
                    return Some(id.as_i64().unwrap());
                }
            }

            for value in map.values() {
                if let Some(id) = find_chat_id(value) {
                    return Some(id);
                }
            }

            None
        }
        Value::Array(array) => {
            for value in array {
                if let Some(id) = find_chat_id(value) {
                    return Some(id);
                }
            }

            None
        }
        _ => None,
    }
}