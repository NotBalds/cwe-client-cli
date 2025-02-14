use super::cfg::*;

pub fn storage() -> bool {
    if APP_DIR.exists()
        && KEYS_DIR.exists()
        && STORAGE_DIR.exists()
        && CONFIG_PATH.exists()
        && KEYS_DIR.join("sys-key").exists()
        && KEYS_DIR.join("sys-key.pub").exists()
        && KEYS_DIR.join("my-key").exists()
        && KEYS_DIR.join("my-key.pub").exists()
        && KEYS_DIR.join("nonce").exists()
    {
        return true;
    } else {
        return false;
    }
}

pub fn server(url: String, port: String) -> Result<bool, crate::AnyError> {
    Ok(true)
}
