use crate::base;
use serde::{Deserialize, Serialize};

pub struct Contact {
    pub name: String,
    pub uuid: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostRegister {
    pub uuid: String,
    pub publickey: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostGet {
    pub uuid: String,
    pub gettime: String,
    pub gettimesignature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub format: String,
    pub info: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: String,
    pub content: Content,
}

pub type GetResponse = Vec<Message>;

#[derive(Serialize, Deserialize)]
pub struct PostSend {
    pub receiver: String,
    pub message: Message,
    pub sendtime: String,
    pub sendtimesignature: String,
}

pub fn to_string<T>(value: &T) -> String
where
    T: Serialize,
{
    match serde_json::to_string(value) {
        Ok(json) => json,
        Err(err) => {
            base::log(&format!("Error while serializing JSON: {}", err), 1);
            String::from("")
        }
    }
}
