
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