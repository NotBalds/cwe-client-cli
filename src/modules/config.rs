use crate::base::filesystem;
use directories::ProjectDirs;
use std::path::PathBuf;

pub const VERSION: &str = "0.2.0";

pub fn path() -> PathBuf {
    ProjectDirs::from("su", "bald", "cwe-client")
        .unwrap()
        .data_local_dir()
        .to_path_buf()
}

pub fn default_url() -> String {
    String::from("http://bald.su:1337/")
}

pub fn url(path: &str) -> String {
    let base = filesystem::cat(&filesystem::new_path("server"));

    base + &path.to_string()
}
