use crate::base::filesystem;
use std::env;
use std::path::PathBuf;

pub const VERSION: &str = "0.1.3";

pub fn path() -> PathBuf {
    let home_dir =
        env::home_dir().expect("IDK what is happened, but i can't find home dir. That's weird");
    let custom_path = home_dir.join(".local").join("share").join("cwe-client");

    custom_path
}

pub fn default_url() -> String {
    String::from("http://bald.su:1337/")
}

pub fn url(path: &str) -> String {
    let base = filesystem::cat(&filesystem::new_path("server"));

    base + &path.to_string()
}
