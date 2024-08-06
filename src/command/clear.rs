use crate::base;
use colored::Colorize;
use std::{
    io::{self, BufRead, Write},
    process::Command,
};

pub fn run() {
    match Command::new("clear").spawn() {
        Ok(_) => (),
        Err(_) => base::log("Can't clear console", 3),
    }
    base::sleep(0.1);
}
