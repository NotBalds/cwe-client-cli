use crate::base;
use std::path::PathBuf;

pub fn run() {
    base::log("Please enter ABSOLUTE path to contact file: ", 5);
    let path = base::correct_input("Path to contact: ", base::filesystem::exist_abs);
    if path == "exit".to_string() {
        return;
    }

    let lines = base::filesystem::cat_lines(&PathBuf::from(path.as_str()));
    let contact_server_host = lines[0].clone();

    if contact_server_host.clone() != base::config::default_url() {
        base::log("Sorry, but this contact is not from your server", 3);
    } else {
        let contact_uuid = lines[1].clone();
        let mut contact_public_key = String::from("");
        for line in &lines[2..] {
            contact_public_key = contact_public_key + line + "\n";
        }

        fn check(string: String) -> bool {
            let forbidden_chars = vec![
                " ", "\n", "\r", "\t", "/", "\\", ":", "*", "?", "<", ">", "|", "&", "$", "!", "'",
                "\"", "`", "(", ")", "{", "}", "[", "]",
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

        base::log("How would you like to call this user?", 5);
        let contact_name = base::correct_input("Enter name: ", check);
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
}
