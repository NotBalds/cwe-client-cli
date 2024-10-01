use crate::sys;
use std::env;

pub fn run(args: env::Args) {
    if args.len() > 1 {
    } else {
        let path = sys::files::core::path::Path::new("~");
        let list = match sys::files::core::path::ls(&path.path) {
            Ok(list) => list,
            Err(err) => panic!("Error: {}", err),
        };

        for obj in list {
            println!("{}", obj);
        }
    }
}
