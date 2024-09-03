use crate::{base, modules};
use std::path::PathBuf;

pub fn run(passphrase: String) {
    fn check(string: String) -> bool {
        let forbidden_chars = vec![
            " ", "\n", "\r", "\t", "\\", ":", "*", "?", "<", ">", "|", "&", "$", "!", "'", "\"",
            "`", "(", ")", "{", "}", "[", "]",
        ];
        for forbidden_char in &forbidden_chars {
            if string.contains(forbidden_char) {
                return false;
            }
        }
        return true;
    }
    let stored_messages_from = base::filesystem::ls(
        base::filesystem::new_path("history")
            .as_path()
            .to_str()
            .unwrap(),
    );

    let mut senders_names = vec![];
    for sender_uuid in stored_messages_from.clone() {
        let sender_name = base::contact::get_name(sender_uuid);
        senders_names.push(sender_name);
    }

    let choice = base::get_choice(senders_names.clone(), "Select contact to view history: ");
    if choice == -1 {
        return;
    }
    let sender_uuid = &stored_messages_from[choice as usize].clone();
    let sender_name = senders_names[choice as usize].clone().to_string();

    let mut sender = base::contact::get(sender_name);
    if sender.uuid == "" {
        sender.uuid = sender_uuid.clone();
    }

    let user_path = format!("history/{}", sender.uuid);
    let messages_names = base::filesystem::ls(
        base::filesystem::new_path(&user_path)
            .as_path()
            .to_str()
            .unwrap(),
    );

    let mut messages_info: Vec<String> = vec![];
    for message_name in messages_names {
        messages_info.push(message_name);
    }
    messages_info.sort_by(|a, b| b.cmp(a));

    let choice = base::get_choice(messages_info.clone(), "Select message to view: ");
    if choice == -1 {
        return;
    }
    let message_info = messages_info[choice as usize].clone();
    let message_info_vec = message_info.split("|").collect::<Vec<&str>>();
    let meta = message_info_vec[0].split("_").collect::<Vec<&str>>();

    if meta[1] == "text" {
        if meta[2] == "safe" {
            let encrypted_text = base::filesystem::cat_lines(&base::filesystem::new_path(
                &format!("history/{}/{}", sender.uuid, message_info),
            ));

            let decrypted_text =
                modules::crypting::base::decrypt(encrypted_text, passphrase.clone());
            let text = String::from_utf8(decrypted_text)
                .unwrap_or("[COULDN'T READ TEXT FROM UTF-8]".to_string());

            base::log("Content: ", 4);
            base::log(&text, 4);
        } else if meta[2] == "unsafe" {
            let text = base::filesystem::cat(&base::filesystem::new_path(&format!(
                "history/{}/{}",
                sender.uuid, message_info
            )));
            base::log("Content: ", 4);
            base::log(&text, 4);
        } else {
            base::log("Info about crypting not found!", 3);
            base::log("Couldn't view message", 6);
        }
    } else if meta[1] == "file" {
        if meta[2] == "safe" {
            let encrypted_file = base::filesystem::cat_lines(&base::filesystem::new_path(
                &format!("history/{}/{}", sender.uuid, message_info),
            ));
            let file = modules::crypting::base::decrypt(encrypted_file, passphrase.clone());

            let path = base::correct_input("Select place to extract data: ", check);
            base::filesystem::becho(file, &PathBuf::from(path));

            base::log("Extracted!", 0);
        } else if meta[2] == "unsafe" {
            let file = base::filesystem::bcat(
                base::filesystem::new_path(&format!("history/{}/{}", sender.uuid, message_info))
                    .as_path()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );

            let path = base::correct_input("Select place to extract data: ", check);
            let path = base::config::tilda_to_abs_path(path);

            base::filesystem::becho(file, &PathBuf::from(path));

            base::log("Extracted!", 0);
        } else {
            base::log("Info about crypting not found!", 3);
            base::log("Couldn't view message", 6);
        }
    } else {
        base::log("Info about format not found!", 3);
        base::log("Couldn't view message", 6);
    }
}
