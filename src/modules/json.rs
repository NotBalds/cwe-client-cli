use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct Message {
    pub sender: String,
    pub content: String,
    pub content_type: String,
}

pub type GetResponse = Vec<Message>;

#[derive(Serialize, Deserialize)]
pub struct PostSend {
    pub receiver: String,
    pub sender: String,
    pub content: String,
    pub content_type: String,
    pub sendtime: String,
    pub sendtimesignature: String,
}

pub fn to_hashmap<Type: Serialize>(obj: &Type) -> HashMap<String, String> {
    let json_value = serde_json::to_value(obj).unwrap();
    json_value
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| {
            if let Some(value_str) = value.as_str() {
                Some((key.clone(), value_str.to_string()))
            } else {
                None
            }
        })
        .collect()
}
