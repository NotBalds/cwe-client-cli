use {
    crate::base,
    openssl::{
        rand::rand_bytes,
        symm::{Cipher, Crypter, Mode},
    },
};

pub struct Key {
    pub key: Vec<u8>,
    pub iv: Vec<u8>,
}

pub fn gen_key() -> Key {
    let mut key = vec![0u8; 32];
    let mut iv = vec![0u8; 16];

    rand_bytes(&mut key).unwrap();
    rand_bytes(&mut iv).unwrap();

    Key { key, iv }
}

pub fn encrypt(data: Vec<u8>, key: &Key) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = match Crypter::new(cipher, Mode::Encrypt, &key.key, Some(&key.iv)) {
        Ok(crypter) => crypter,
        Err(err) => {
            base::log(&format!("Can't create encrypter: {}", err), 1);
            return vec![];
        }
    };

    let mut encrypted_data = vec![0; data.len() + cipher.block_size()];
    let count = match crypter.update(&data, &mut encrypted_data) {
        Ok(count) => count,
        Err(err) => {
            base::log(&format!("Can't encrypt data: {}", err), 1);
            return vec![];
        }
    };
    let rest = match crypter.finalize(&mut encrypted_data[count..]) {
        Ok(rest) => rest,
        Err(err) => {
            base::log(&format!("Can't finalize encrypter: {}", err), 1);
            return vec![];
        }
    };

    encrypted_data.truncate(count + rest);
    encrypted_data
}

pub fn decrypt(encrypted_data: Vec<u8>, key: &Key) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = match Crypter::new(cipher, Mode::Decrypt, &key.key, Some(&key.iv)) {
        Ok(crypter) => crypter,
        Err(err) => {
            base::log(&format!("Can't create encrypter: {}", err), 1);
            return vec![];
        }
    };

    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];
    let count = match crypter.update(&encrypted_data, &mut decrypted_data) {
        Ok(count) => count,
        Err(err) => {
            base::log(&format!("Can't encrypt data: {}", err), 1);
            return vec![];
        }
    };
    let rest = match crypter.finalize(&mut decrypted_data[count..]) {
        Ok(rest) => rest,
        Err(err) => {
            base::log(&format!("Can't finalize encrypter: {}", err), 1);
            return vec![];
        }
    };

    decrypted_data.truncate(count + rest);
    decrypted_data
}
