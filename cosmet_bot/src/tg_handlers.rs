use crate::tg_objects::Message;
use crate::application::Application;
use crate::tg_bot::{send_msg, MsgRequest};
use crate::tg_utils::{CommandType, command_str_to_type, MsgType};
use serde_json::Value;
use log::{debug, warn, error};

pub async fn handle_message(app : Application, response_results: &Vec<Value>, offset: &mut i64) -> Result<(), Box<dyn std::error::Error>>
{
    for res in response_results {
        if res.get("message").is_some() && 
           res["message"].is_object() && 
           res["message"].as_object().and_then(|m| m.get("sticker")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("photo")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("animation")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("video")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("voice")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("video_note")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("document")).is_none() &&
           res["message"].as_object().and_then(|m: &serde_json::Map<String, Value>| m.get("location")).is_none() &&
           res["message"].as_object().and_then(|m: &serde_json::Map<String, Value>| m.get("poll")).is_none() &&
           res["message"].as_object().and_then(|m: &serde_json::Map<String, Value>| m.get("contact")).is_none() &&
           res["message"].as_object().and_then(|m: &serde_json::Map<String, Value>| m.get("audio")).is_none() &&
           res["message"].as_object().and_then(|m: &serde_json::Map<String, Value>| m.get("new_chat_member")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("group_chat_created")).is_none() &&
           res["message"].as_object().and_then(|m| m.get("entities")).is_none() &&
           res["message"]["chat"].as_object().and_then(|m| m.get("all_members_are_administrators")).is_none() {
            debug!("{:#?}", res);
            let msg_obj: Value = serde_json::from_str(res["message"].to_string().as_str()).unwrap();
            let msg: Message = serde_json::from_value(msg_obj).unwrap();
            let mut req :  MsgRequest = 
                MsgRequest::new(app.clone(), res["update_id"].as_i64().unwrap(), MsgType::SendMessage, msg);

            // Check if the message is a command
            if req.msg.text.starts_with("/") 
            {
                if req.msg.text.len() == 1 {
                    handle_command(offset, None, &mut req).await?;
                    continue;
                }
                let command = req.msg.text[1..].split_whitespace().next().unwrap();
                debug!("Get {} command", command);
                handle_command(offset, command_str_to_type(command), &mut req).await?;
                continue;
            }
            req.msg.text = "Is not a command".to_string();
            send_msg(offset, &mut req).await?;
            continue;
        }
        else if res.get("my_chat_member").is_some() {
            debug!("Unknown command my_chat_member {:#?}", res);
            let chat_id = res["my_chat_member"]["chat"]["id"].as_i64().unwrap();
            let from_first_name = res["my_chat_member"]["from"]["first_name"].as_str().unwrap();

            let mut msg: Message = Message::new(chat_id, from_first_name);
            msg.text = "Unknown command".to_string();
            let mut req :  MsgRequest = 
                MsgRequest::new(app.clone(), res["update_id"].as_i64().unwrap(), MsgType::SendMessage, msg);
            req.msg.text = "Пиздец че ты ещё придумаешь".to_string();
            send_msg(offset, &mut req).await?;
        }
        else {

            debug!("Unknown command {:#?}", res);
            
            let chat_id = match res.get("message") {
                Some(_) => res["message"]["chat"]["id"].as_i64().unwrap(),
                None => res["edited_message"]["chat"]["id"].as_i64().unwrap(),
            };

            let from_first_name = match res.get("message") {
                Some(_) => res["message"]["from"]["first_name"].as_str().unwrap(),
                None => res["edited_message"]["from"]["first_name"].as_str().unwrap(),
            };
            let mut msg: Message = Message::new(chat_id, from_first_name);
            msg.text = "Unknown command".to_string();
            let mut req :  MsgRequest = 
                MsgRequest::new(app.clone(), res["update_id"].as_i64().unwrap(), MsgType::SendMessage, msg);
            req.msg.text = "Wrong command son of whore".to_string();
            send_msg(offset, &mut req).await?;
            continue;
        }
    }
    Ok(())
}

async fn handle_command(offset: &mut i64, command_t : Option<CommandType>, req: &mut MsgRequest) -> Result<serde_json::Value, reqwest::Error> 
{
    match command_t {
        Some(CommandType::Hello) => handle_hello_command(offset, req).await,
        None => handle_unknown_command(offset, req).await,
    }
}

async fn handle_hello_command(offset: &mut i64, req: &mut MsgRequest) -> Result<serde_json::Value, reqwest::Error> 
{
    debug!("Hello command was called");
    req.msg.text = "Hello command was called".to_string();
    send_msg(offset, req).await
}

async fn handle_unknown_command(offset: &mut i64, req: &mut MsgRequest) -> Result<serde_json::Value, reqwest::Error> 
{
    warn!("Unknown command was called");
    req.msg.text = "Unknown command was called".to_string();
    send_msg(offset, req).await
}