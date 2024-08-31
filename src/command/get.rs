use crate::{base, modules};
pub use base64::{prelude::BASE64_STANDARD as b64, Engine};
use std::collections::BTreeMap;

pub fn run(passphrase: String) {
    base::log("Getting messages...", 2);

    let uuid = base::uuid::get();
    let sendtime = base::unix_time().to_string();
    let sendtimesignature = modules::crypting::rsa::sign(sendtime.clone(), passphrase.clone());

    let (status_code, encrypted_data_blocks) =
        modules::network::get(uuid, sendtime, sendtimesignature);

    if status_code == 0 {
        base::log("Couldn't get messages", 3);
        return;
    } else if encrypted_data_blocks.clone().len() == 0 {
        base::log("No messages found", 0);
        return;
    } else {
        base::log("Got messages", 0);

        let encrypted_messages: BTreeMap<String, BTreeMap<String, Vec<String>>> =
            to_encrypted_messages(encrypted_data_blocks.clone(), passphrase.clone());
        let decrypted_messages = to_messages(encrypted_messages, passphrase.clone());

        for (sender_uuid, messages) in decrypted_messages.clone() {
            let me = base::contact::get_me();
            let sender_name = base::contact::get_name(sender_uuid.clone());
            let sender = match sender_name.as_str() {
                "Unknown" => {
                    let mut sender = base::contact::get(sender_name);
                    sender.uuid = sender_uuid;
                    sender
                }
                _ => base::contact::get(sender_name),
            };
            for (message_info, message) in messages {
                let mut message_info = message_info.as_str().split("|");
                let message_format = message_info.next().unwrap();
                let message_uuid = message_info.next().unwrap();
                let _message_total_blocks = message_info.next().unwrap();
                let message_sendtime = message_info.next().unwrap();
                let message_info = message_info.next().unwrap();

                let save_message_from_user_path =
                    base::filesystem::new_path("history").join(sender.uuid.clone());

                if !save_message_from_user_path.exists() {
                    match base::filesystem::mkAllDirs(&save_message_from_user_path) {
                        Ok(()) => (),
                        Err(err) => base::log(
                            &format!(
                                "Error while trying to create {}: {}",
                                save_message_from_user_path.to_str().unwrap(),
                                err
                            ),
                            1,
                        ),
                    };
                }

                let output_message_filename = format!(
                    "{}_{}_{}_{}|{}",
                    message_sendtime,
                    message_format,
                    if base::config::SAFE_HISTORY {
                        "safe"
                    } else {
                        "unsafe"
                    },
                    message_uuid,
                    message_info
                );

                let save_message_path = save_message_from_user_path.join(&output_message_filename);
                if base::config::SAFE_HISTORY {
                    let message =
                        modules::crypting::base::encrypt(message, me.public_key.clone()).0;
                    let mut file_data = String::new();
                    for block in message {
                        file_data.push_str(&format!("{}\n", block));
                    }
                    base::filesystem::echo(file_data, &save_message_path);
                } else {
                    base::filesystem::becho(message, &save_message_path);
                }
            }
        }

        for (sender_uuid, messages) in decrypted_messages.clone() {
            let sender_name = base::contact::get_name(sender_uuid.clone());
            let mut sender = base::contact::get(sender_name);
            sender.uuid = sender_uuid;

            base::log(
                &format!("Messages from {} - {}", sender.name, sender.uuid),
                4,
            );
            for (message_info, message) in messages {
                let mut message_info = message_info.as_str().split("|");
                let message_format = message_info.next().unwrap();
                let _message_uuid = message_info.next().unwrap();
                let message_total_blocks = message_info.next().unwrap();
                let _message_sendtime = message_info.next().unwrap();
                let message_info = message_info.next().unwrap();

                println!(
                    " |- Blocks: {} \n |- Info: {} \n |- Content: ",
                    message_total_blocks, message_info,
                );

                if message_format == "text" {
                    base::log(
                        &String::from_utf8(message.clone())
                            .unwrap_or("[COULDN'T READ TEXT FROM UTF-8]".to_string()),
                        4,
                    );
                } else if message_format == "file" {
                    println!("[FILE DATA]");
                }
            }
        }
    }
}

fn to_encrypted_messages(
    encrypted_data_blocks: Vec<modules::json::Message>,
    passphrase: String,
) -> BTreeMap<String, BTreeMap<String, Vec<String>>> {
    base::log("Parsing encrypted data blocks...", 2);
    let mut encrypted_messages: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for encrypted_data_block in encrypted_data_blocks {
        if encrypted_messages
            .get(&encrypted_data_block.sender.clone())
            .is_none()
        {
            encrypted_messages.insert(encrypted_data_block.sender.clone(), BTreeMap::new());
        }

        let encrypted_data_block_info = match String::from_utf8(modules::crypting::rsa::decrypt(
            match b64.decode(encrypted_data_block.content.info) {
                Ok(info) => info,
                Err(_) => Vec::new(),
            },
            passphrase.clone(),
        )) {
            Ok(info) => info,
            Err(_) => String::from(""),
        };

        if encrypted_messages
            .get(&encrypted_data_block.sender.clone())
            .unwrap()
            .get(&encrypted_data_block_info.clone())
            .is_none()
        {
            encrypted_messages
                .get_mut(&encrypted_data_block.sender.clone())
                .unwrap()
                .insert(encrypted_data_block_info.clone(), Vec::new());
        }

        let encrypted_message = encrypted_messages
            .get_mut(&encrypted_data_block.sender.clone())
            .unwrap()
            .get_mut(&encrypted_data_block_info.clone())
            .unwrap();

        encrypted_message.push(encrypted_data_block.content.data.clone());
    }
    base::log("Parsed encrypted data blocks", 0);
    encrypted_messages
}

fn to_messages(
    encrypted_messages: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    passphrase: String,
) -> BTreeMap<String, BTreeMap<String, Vec<u8>>> {
    base::log("Decrypting messages...", 2);
    let mut messages: BTreeMap<String, BTreeMap<String, Vec<u8>>> = BTreeMap::new();

    for (sender, encrypted_messages_from_one) in encrypted_messages {
        for (encrypted_message_info, encrypted_message) in encrypted_messages_from_one {
            let decrypted_message =
                modules::crypting::base::decrypt(encrypted_message, passphrase.clone());
            if messages.get(&sender.clone()).is_none() {
                messages.insert(sender.clone(), BTreeMap::new());
            }
            if messages
                .get(&sender.clone())
                .unwrap()
                .get(&encrypted_message_info.clone())
                .is_none()
            {
                messages
                    .get_mut(&sender.clone())
                    .unwrap()
                    .insert(encrypted_message_info.clone(), Vec::new());
            }
            messages
                .get_mut(&sender.clone())
                .unwrap()
                .get_mut(&encrypted_message_info)
                .unwrap()
                .extend_from_slice(decrypted_message.as_slice());
        }
    }

    base::log("Decrypted messages", 0);

    messages
}
