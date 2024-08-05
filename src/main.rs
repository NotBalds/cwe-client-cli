use std::env;

mod app;
mod base;
mod modules;

fn main() {
    let args = env::args();
    app::run(args);
}
