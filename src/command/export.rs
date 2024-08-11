use crate::base;
use std::path::PathBuf;

pub fn run() {
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
        if !PathBuf::from(string).is_absolute() {
            return false;
        }
        return true;
    }
    loop {
        base::log(
            "Please enter ABSOLUTE path to export file: (example: /home/user/me.contact)",
            5,
        );
        let export_path = base::input("Export path: ");
        if check(export_path.clone()) {
            let server_host = base::config::default_url();
            let uuid = base::uuid::get();
            let public_key =
                base::filesystem::cat(&base::filesystem::new_path("base-keys/my-key.pub"));

            base::filesystem::echo(
                server_host + "\n" + &uuid + "\n" + &public_key,
                &PathBuf::from(export_path),
            );

            base::log("Successfully exported your contact!", 0);

            break;
        } else {
            base::log("Sorry, but this name is invalid", 3);
        }
    }
}
