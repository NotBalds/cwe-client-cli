use cwe_client_cli::app::init;
use std::env;

fn main() {
    let _args = env::args();
    init::run();
}
