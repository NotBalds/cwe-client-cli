use crate::{base, modules::crypting::correct_passphrase};
use rpassword;

pub fn get(prompt: &str) -> String {
    match rpassword::prompt_password(prompt) {
        Ok(passphrase) => passphrase,
        Err(err) => {
            base::log(&format!("Error: {}", err), 1);
            String::new()
        }
    }
}

pub fn check(prompt: &str) -> (bool, String) {
    let mut input = String::new();
    for _ in 1..=3 {
        input = get(prompt);
        if correct_passphrase(&input) {
            return (true, input);
        }
    }
    correct_passphrase(&input);
    (false, String::new())
}

pub fn create() -> String {
    loop {
        base::log("Enter new passphrase", 5);
        let passphrase1 = get("Passphrase: ");
        let passphrase2 = get("Re-enter passphrase: ");
        if passphrase1 == passphrase2 {
            return passphrase1;
        } else {
            base::log("Passphrases do not match. Try again!", 3);
        }
    }
}
