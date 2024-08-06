use crate::base;
use std::process::Command;

pub fn run() {
    match Command::new("clear").spawn() {
        Ok(_) => (),
        Err(_) => base::log("Can't clear console", 3),
    }
    base::sleep(0.1);
}
