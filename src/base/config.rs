#![allow(deprecated)]

use crate::base::filesystem;
use directories::ProjectDirs;
use std::path::PathBuf;

pub const VERSION: &str = "0.3.1-dev";
pub const DEV_MODE: bool = true;
pub const SAFE_HISTORY: bool = true;

pub const SYS_BITS: u32 = 2048;
pub const BASE_BITS: u32 = 4096;
pub const BLOCK_BITS: u32 = 4096000;

pub const SUPPORTED_FORMATS: [&str; 2] = ["text", "file"];

pub fn linux_home_path() -> String {
    std::env::home_dir().unwrap().to_str().unwrap().to_string()
}

pub fn tilda_to_abs_path(path_with_tilda: String) -> String {
    let mut path_with_tilda = path_with_tilda;
    if &path_with_tilda.chars().nth(0).unwrap() == &'~' {
        path_with_tilda = linux_home_path() + &path_with_tilda[1..];
    }
    path_with_tilda
}

pub fn path() -> PathBuf {
    let result_path = ProjectDirs::from("su", "bald", "cwe-client")
        .unwrap()
        .data_local_dir()
        .to_path_buf();
    match DEV_MODE {
        true => result_path.join("dev"),
        false => result_path,
    }
}

pub fn default_url() -> String {
    String::from("http://127.0.0.1:1337/")
}

pub fn supported_formats() -> Vec<String> {
    SUPPORTED_FORMATS.iter().map(|x| x.to_string()).collect()
}

pub fn url(path: &str) -> String {
    let base = filesystem::cat(&filesystem::new_path("server"));

    base + &path.to_string()
}
