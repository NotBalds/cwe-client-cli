use crate::{base, modules};
pub use base64::{prelude::BASE64_STANDARD as b64, Engine};

pub fn run(passphrase: String) {
    let contacts = base::contact::get_list();
    let choice = base::get_choice(contacts.clone(), "Select contact: ");
    if choice == -1 {
        return;
    }
    let contact = base::contact::get(contacts[choice as usize].clone());

    let supported_formats = base::config::supported_formats();
    let choice = base::get_choice(supported_formats.clone(), "Select format: ");
    if choice == -1 {
        return;
    }
    let format = supported_formats[choice as usize].clone();

    let mut additional_info: String = String::new();
    let mut data: Vec<u8> = vec![];
    match format.as_str() {
        "text" => {
            let message = base::input("Enter message: ");
            data.extend_from_slice(message.as_bytes());
            additional_info.push_str("text");
        }
        "file" => {
            fn check(path: String) -> bool {
                let path = base::config::tilda_to_abs_path(path);
                if base::filesystem::get_file_name(path.clone()).contains('|') {
                    false
                } else {
                    base::filesystem::exist_abs(path)
                }
            }

            let path = base::correct_input("Enter path to file: ", check);
            let file_name = base::filesystem::get_file_name(path.clone());
            let file_data = base::filesystem::bcat(path.clone());
            data.extend_from_slice(&file_data);
            additional_info.push_str(&file_name);
        }
        _ => {
            base::log(&format!("Format {} not supported!", format), 1);
        }
    }

    let (encrypted_data_blocks, total_blocks) =
        modules::crypting::base::encrypt(data, contact.public_key.clone());

    let info = format!(
        "{}|{}|{}|{}|{}",
        format.clone(),
        base::uuid::generate(),
        total_blocks,
        base::unix_time(),
        additional_info,
    );
    let info =
        modules::crypting::rsa::encrypt(info.as_bytes().to_vec(), contact.public_key.clone())
            .clone();
    let info = b64.encode(&info);

    let format =
        modules::crypting::rsa::encrypt(format.as_bytes().to_vec(), contact.public_key.clone())
            .clone();
    let format = b64.encode(&format);

    let sender: String = base::uuid::get();

    base::log("Sending message...", 2);

    let mut current_encrypted_data_block: u128 = 0;
    for encrypted_data_block in encrypted_data_blocks {
        current_encrypted_data_block += 1;

        let message = modules::json::Message {
            sender: sender.clone(),
            content: modules::json::Content {
                format: format.clone(),
                info: info.clone(),
                data: encrypted_data_block,
            },
        };

        let sendtime = base::unix_time().to_string();
        let sendtimesignature = modules::crypting::rsa::sign(sendtime.clone(), passphrase.clone());

        let status_code =
            modules::network::send(contact.uuid.clone(), message, sendtime, sendtimesignature);
        if status_code != 200 {
            base::log(
                &format!("Couldn't send message. Error code: {}", status_code),
                6,
            );
            base::log("Couldn't send message", 6);
            return;
        }

        base::log(
            &format!(
                "Sent {}/{}",
                current_encrypted_data_block.clone(),
                total_blocks.clone()
            ),
            0,
        );
    }

    base::log("Message sent!", 0);
}
