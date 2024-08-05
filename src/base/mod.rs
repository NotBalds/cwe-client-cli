pub mod check;
pub mod contact;
pub mod filesystem;
pub mod passphrase;
pub mod prepare;
pub mod uuid;

use colored::Colorize;
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str, t: i8) {
    match t {
        0 => {
            let sign = "[+]";
            println!("{} {}", sign.green().bold(), msg.white());
        }
        1 => {
            let sign = "[-]";
            println!("{} {}", sign.red().bold(), msg.red());
            panic!();
        }
        2 => {
            let sign = "[.]";
            println!("{} {}", sign.bold(), msg.white());
        }
        3 => {
            let sign = "[!]";
            println!("{} {}", sign.yellow().bold(), msg.yellow());
        }
        4 => {
            let sign = "[?]";
            println!("{} {}", sign.bold(), msg.white());
        }
        5 => {
            let sign = "[>]";
            println!("{} {}", sign.bold().blue(), msg.white());
        }
        _ => {}
    }
}

pub fn input(prompt: &str) -> String {
    print!("{} {}", "[>]".bold().blue(), prompt);
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => log(&format!("Error flushing: {}", e.to_string()), 1),
    };

    match io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
    {
        Ok(input) => input,
        Err(err) => {
            log(&format!("Error reading input: {}", err.to_string()), 1);
            String::new()
        }
    }
}

pub fn unix_time() -> u64 {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let unix_time = duration.as_secs();
    unix_time
}

pub fn sleep(seconds: f32) {
    std::thread::sleep(std::time::Duration::from_millis((seconds * 1000.0) as u64));
}
