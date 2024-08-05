use crate::base;
use std::env;

pub fn run(args: env::Args) {
    let status: i8 = base::check::run();

    if status == 0 {
        base::log("Please enter passphrase: ", 5);
        if !base::passphrase::check("Passphrase: ") {
            return;
        };
    } else if status == 1 {
        match base::input("Would you like to create client directory? (Y/n): ").as_str() {
            "y" | "Y" | "" => {
                let passphrase = base::passphrase::create();
                match base::prepare::run(passphrase, false) {
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
                match base::prepare::run(base::passphrase::create(), true) {
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

    if args.len() > 1 {
    } else {
        base::sleep(0.5);
        println!();
        base::log("Welcome to CWE Command Line Interface!", 4);
        base::log("Please enter what you want to do: ", 4);
        base::log("You can enter 'help' to get list of commands", 4);
    }
}
