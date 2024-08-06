use crate::{base, command};
use std::env;

pub fn run(args: env::Args) {
    if args.len() > 1 {
        base::log("Sorry but non-interactive part is not ready", 1);
    } else {
        let status: i8 = base::check::run();
        let mut correct_passphrase = String::new();

        if status == 0 {
            base::log("Please enter passphrase: ", 5);
            let result = base::passphrase::check("Passphrase: ");
            if !result.0 {
                return;
            };
            correct_passphrase = result.1;
        } else if status == 1 {
            match base::input("Would you like to create client directory? (Y/n): ").as_str() {
                "y" | "Y" | "" => {
                    correct_passphrase.push_str(base::passphrase::create().as_str());
                    match base::prepare::run(correct_passphrase.clone(), false) {
                        Ok(_) => (),
                        Err(err) => base::log(&format!("Error: {}", err), 1),
                    };
                }
                _ => {
                    return;
                }
            }
        } else if status == 2 {
            base::log("Found files may be not compatible!", 3);
            match base::input("Would you like to recreate client directory? (Y/n): ").as_str() {
                "Y" | "" | "y" => {
                    correct_passphrase.push_str(base::passphrase::create().as_str());
                    match base::prepare::run(correct_passphrase.clone(), true) {
                        Ok(_) => (),
                        Err(err) => base::log(&format!("Error: {}", err), 1),
                    };
                }
                "n" | "N" => {
                    base::log("Continued without recreating client directory", 3);
                }
                _ => {
                    return;
                }
            };
        }

        base::sleep(0.5);
        println!();
        base::log("Welcome to CWE Command Line Interface!", 4);

        loop {
            base::log("Please enter what you want to do: ", 4);
            base::log("You can enter 'help' to get list of commands", 4);

            let command: String = base::input("$ ");

            if command == "exit" {
                break;
            } else if command == "help" {
                command::help::run();
            } else if command == "" {
            } else if command == "info" {
                command::info::run();
            } else if command == "import" {
                command::import::run();
            } else if command == "export" {
                command::export::run();
            } else if command == "send" {
                command::send::run(correct_passphrase.clone());
            } else if command == "get" {
                command::get::run(correct_passphrase.clone());
            } else if command == "del" {
                command::del::run();
            } else {
                base::log(&format!("Sorry, but {} is not an option", command), 3);
            }
        }
    }
}
