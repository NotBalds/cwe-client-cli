use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;

// Basic
pub const VERSION: &str = "0.9.0";

// Net
// - Protocols
pub const HTTP_PROTOCOL: &str = "http";
pub const WS_PROTOCOL: &str = "ws";
pub const SUPPORTED_PROTOCOLS: [&str; 2] = [HTTP_PROTOCOL, WS_PROTOCOL];

pub const DEFAULT_PROTOCOL: &str = HTTP_PROTOCOL;

// - Server
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: &str = "1337";

// - Storage
pub static APP_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("su", "bald", "cwe-client")
        .expect("Could not determine project directory")
        .data_dir()
        .to_path_buf()
});

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = APP_DIR.clone();
    path.push("config.toml");
    path
});

pub static KEYS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = APP_DIR.clone();
    path.push("keys");
    path
});

pub static STORAGE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = APP_DIR.clone();
    path.push("storage");
    path
});
