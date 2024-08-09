use openssl::{
    rand::rand_bytes,
    symm::{Cipher, Crypter, Mode},
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
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key.key, Some(&key.iv)).unwrap();

    let mut encrypted_data = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(&data, &mut encrypted_data).unwrap();
    let rest = crypter.finalize(&mut encrypted_data[count..]).unwrap();

    encrypted_data.truncate(count + rest);
    encrypted_data
}

pub fn decrypt(encrypted_data: Vec<u8>, key: &Key) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key.key, Some(&key.iv)).unwrap();

    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];
    let count = crypter
        .update(&encrypted_data, &mut decrypted_data)
        .unwrap();
    let rest = crypter.finalize(&mut decrypted_data[count..]).unwrap();

    decrypted_data.truncate(count + rest);
    decrypted_data
}
