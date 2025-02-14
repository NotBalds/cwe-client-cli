use crate::AnyError;
use crate::{app, sys};
use notblib::{crypt, fs, get, stat};

pub fn run() -> Result<(), crate::AnyError> {
    stat::process("Preparing for work...");

    if !sys::check::storage() {
        stat::fatal("App storage is corrypted!");
        if get::agreement("Would you reset app storage? [y/n] ")? {
            reset()?;
            stat::success("Reset completed!");
        }
    }

    stat::success("Ready for work!");
    Ok(())
}

fn reset() -> Result<(), AnyError> {
    // Deleting all
    if sys::cfg::APP_DIR.exists() {
        stat::process("Deleting old app storage PERMANENTLY!");
        fs::delete::dir_all(&sys::cfg::APP_DIR)?;
        stat::success("Deleted =), have a nice day");
    }

    // Creating app dir
    fs::create::dir(&sys::cfg::APP_DIR)?;

    // Creating config
    {
        stat::process("Creating new config...");
        fs::create::file(&sys::cfg::CONFIG_PATH)?;

        let mut config = app::cfg::Config {
            version: sys::cfg::VERSION.into(),
            net: app::cfg::NetConfig {
                host: sys::cfg::DEFAULT_HOST.into(),
                port: sys::cfg::DEFAULT_PORT.into(),
                proto: sys::cfg::DEFAULT_PROTOCOL.into(),
            },
        };
        config.set()?;
        stat::success("Default config saved!");

        // Getting server host
        let host = loop {
            let input = get::input(format!(
                "Enter server host or ip (ex. 'bald.su' without port or proto).\n    Just type enter if want to save default ({})\n    Type: ",
                sys::cfg::DEFAULT_HOST
            ))?;

            let host = match input.as_str() {
                "" => sys::cfg::DEFAULT_HOST.to_string(),
                any_else => any_else.to_string(),
            };

            if sys::check::server(host.clone(), sys::cfg::DEFAULT_PORT.to_string())? {
                break host;
            } else {
                stat::error("Server failed checking! Try another one");
                continue;
            }
        };

        config.net.host = host;
        config.set()?;
        stat::success("Server saved!")
    }

    // Setting basic crypting
    fs::create::dir(&sys::cfg::KEYS_DIR)?;

    // Getting key from passphrase
    let key = {
        let mut passphrase = loop {
            let secret = get::secret("Enter passphrase: ")?;
            let secret_again = get::secret("Enter passphrase again: ")?;

            if secret != secret_again {
                stat::error("Passwords mismatch!");
                continue;
            } else {
                break secret;
            }
        }
        .as_bytes()
        .to_vec();

        fn pad_vec_to_32(data: Vec<u8>) -> Vec<u8> {
            let mut data = data.clone();
            if data.len() < 32 {
                data.resize(32, 0)
            }
            data
        }
        passphrase = pad_vec_to_32(passphrase);

        let key_bytes = crypt::sha256::hash(passphrase.as_slice());
        let nonce = {
            use randomizer::{Charset, Randomizer};

            let bytes = Randomizer::new(12, Some(Charset::AnyByte)).bytes().unwrap();
            let mut nonce_array = [0u8; 12];
            nonce_array.copy_from_slice(&bytes[0..12]);
            nonce_array
        };

        fs::write::bytes(&sys::cfg::KEYS_DIR.join("nonce"), nonce.to_vec())?;

        crypt::aes_256::Key::from((key_bytes, nonce))
    };

    // Gen & Save sys-keys
    {
        let ecc_key = crypt::elliptic::gen_keys();
        let ecc_key_secret_encrypted = crypt::aes_256::encrypt(&key, ecc_key.secret.to_vec())?;
        let ecc_key_public_encrypted = crypt::aes_256::encrypt(&key, ecc_key.public.to_vec())?;

        fs::write::bytes(
            &sys::cfg::KEYS_DIR.join("sys-key"),
            ecc_key_secret_encrypted,
        )?;
        fs::write::bytes(
            &sys::cfg::KEYS_DIR.join("sys-key.pub"),
            ecc_key_public_encrypted,
        )?;
    }

    // Gen & Save my-keys
    {
        let ecc_key = crypt::elliptic::gen_keys();
        let ecc_key_secret_encrypted = crypt::aes_256::encrypt(&key, ecc_key.secret.to_vec())?;
        let ecc_key_public_encrypted = crypt::aes_256::encrypt(&key, ecc_key.public.to_vec())?;

        fs::write::bytes(&sys::cfg::KEYS_DIR.join("my-key"), ecc_key_secret_encrypted)?;
        fs::write::bytes(
            &sys::cfg::KEYS_DIR.join("my-key.pub"),
            ecc_key_public_encrypted,
        )?;
    }

    fs::create::dir(&sys::cfg::STORAGE_DIR)?;

    Ok(())
}
