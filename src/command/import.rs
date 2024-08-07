use crate::{base, modules};
use std::path::PathBuf;

pub fn run() {
    base::log("Please enter ABSOLUTE path to contact file: ", 5);
    let path = base::input("Path to contact: ");

    if base::filesystem::exist(path.clone().as_str()) {
        let lines = base::filesystem::cat_lines(&PathBuf::from(path.as_str()));
        let contact_server_host = lines[0].clone();

        if contact_server_host.clone() != modules::config::default_url() {
            base::log("Sorry, but this contact is not from your server", 3);
        } else {
            let contact_uuid = lines[1].clone();
            let mut contact_public_key = String::from("");
            for line in &lines[2..] {
                contact_public_key = contact_public_key + line + "\n";
            }

            fn check(string: String) -> bool {
                let forbidden_chars = vec![
                    " ", "\n", "\r", "\t", "/", "\\", ":", "*", "?", "<", ">", "|", "&", "$", "!",
                    "'", "\"", "`", "(", ")", "{", "}", "[", "]",
                ];

                if string == "" {
                    return false;
                }

                for forbidden_char in &forbidden_chars {
                    if string.contains(forbidden_char) {
                        return false;
                    }
                }
                return true;
            }

            let contact_name = base::correct_input("How would you like to call this user?", check);
            if contact_name == "exit" {
                return;
            } else {
                base::filesystem::echo(
                    contact_uuid.clone() + "\n" + &contact_public_key.clone(),
                    &base::filesystem::new_path("contacts").join(contact_name.clone()),
                );
                base::filesystem::echo(
                    contact_name,
                    &base::filesystem::new_path("contacts-uuid").join(contact_uuid.clone()),
                );
            }

            base::log("Contact has been added", 0);
        }
    } else {
        base::log(&format!("Could not find file {}", path), 3);
    }
}
