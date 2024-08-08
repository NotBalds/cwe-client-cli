use crate::{base, modules};
use std::collections::BTreeMap;

pub fn run(passphrase: String) {
    base::log("Getting messages...", 2);

    let uuid = base::uuid::get();
    let gettime = base::unix_time().to_string();
    let gettimesignature = modules::crypting::sign(gettime.clone(), passphrase.clone());

    let (status_code, messages): (u16, modules::json::GetResponse) =
        modules::network::get(uuid.clone(), gettime, gettimesignature);
    base::log(&format!("Status code: {}", status_code), 3);

    let mut ready_messages: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut got_content_size: BTreeMap<String, BTreeMap<String, u128>> = BTreeMap::new();

    for message in messages {
        let sender = message.sender.clone();
        let content_type = match String::from_utf8(modules::crypting::decrypt_data(
            Vec::from([message.content.format]),
            passphrase.clone(),
        )) {
            Ok(content_type) => content_type,
            Err(_) => "Unknown".to_string(),
        };

        let decrypted_content_info =
            modules::crypting::decrypt_data(Vec::from([message.content.info]), passphrase.clone());

        match String::from_utf8(decrypted_content_info) {
            Ok(decrypted_content_info) => {
                let mut parts = decrypted_content_info.split("|");
                let content_uuid = parts.next().unwrap_or("").to_string();
                let content_size = parts.next().unwrap_or("").to_string();
                if ready_messages.get(&sender).is_none() {
                    ready_messages.insert(sender.clone(), BTreeMap::new());
                    got_content_size.insert(sender.clone(), BTreeMap::new());
                }
                let message_identificator = content_uuid.clone()
                    + "|"
                    + content_type.as_str()
                    + "|"
                    + content_size.as_str();
                if ready_messages
                    .get(&sender)
                    .unwrap()
                    .get(&message_identificator.clone())
                    .is_none()
                {
                    ready_messages
                        .get_mut(&sender)
                        .unwrap()
                        .insert(message_identificator.clone(), Vec::new());
                    got_content_size
                        .get_mut(&sender)
                        .unwrap()
                        .insert(message_identificator.clone(), 0);
                }
                let content = ready_messages
                    .get_mut(&sender)
                    .unwrap()
                    .get_mut(&message_identificator.clone())
                    .unwrap();
                let content_size = got_content_size
                    .get_mut(&sender)
                    .unwrap()
                    .get_mut(&message_identificator.clone())
                    .unwrap();
                content.push(message.content.data);
            }
            Err(e) => {
                eprintln!("Convertation content info from UTF8 failed: {}", e);
            }
        }
    }

    for (sender, messages) in ready_messages {
        base::log(
            &format!("Got {} messages from {}", messages.len(), sender),
            0,
        );
        for (message_identificator, content) in messages {
            let mut parts = message_identificator.split("|");
            let content_uuid = parts.next().unwrap_or("").to_string();
            let content_format = parts.next().unwrap_or("").to_string();
            let content_size = match parts.next().unwrap_or("").to_string().parse::<u128>() {
                Ok(content_size) => content_size,
                Err(e) => 0,
            };
            base::log(&format!("Message: {}", message_identificator), 0);
            base::log(
                &format!(
                    "Content: {}",
                    String::from_utf8(modules::crypting::decrypt_data(content, passphrase.clone()))
                        .unwrap()
                ),
                0,
            );
        }
    }
}
