use std::env;

mod app;
mod sys;

fn main() {
    let args = env::args();
    app::run(args);
}
