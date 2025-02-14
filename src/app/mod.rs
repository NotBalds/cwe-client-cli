pub mod cfg;
pub mod cmd;
pub mod init;

use notblib::stat;

pub fn run() {
    match init::run() {
        Err(err) => stat::fatal(err.to_string()),
        _ => match cmd::run() {
            Err(err) => stat::fatal(err.to_string()),
            _ => return,
        },
    };
}
