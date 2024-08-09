use {
    crate::{
        base::{self, config},
        modules::crypting::{aes_256, rsa},
    },
    base64::{prelude::BASE64_STANDARD as b64, Engine},
    openssl::rsa::Rsa,
};

pub fn encrypt(data: Vec<u8>, public_key: String) -> Vec<String> {
    let mut encrypted_data_blocks: Vec<Vec<u8>> = Vec::new();
    let key = aes_256::gen_key();

    base::log("Encrypting symmetric key...", 2);
    let encrypted_key = rsa::encrypt(key.key.clone(), public_key.clone());
    let encrypted_iv = rsa::encrypt(key.iv.clone(), public_key.clone());
    encrypted_data_blocks.push(encrypted_key);
    encrypted_data_blocks.push(encrypted_iv);
    base::log("Symmetric key encrypted", 0);

    base::log("Encrypting data blocks...", 2);
    let data_blocks = split_into_blocks(data, config::BLOCK_BITS as usize);
    let mut current_data_block: u128 = 0;
    for data_block in data_blocks.clone() {
        current_data_block += 1;
        let encrypted_data_block = aes_256::encrypt(data_block, &key);
        encrypted_data_blocks.push(encrypted_data_block);
        base::log(
            &format!(
                "Encrypted block {}/{}",
                current_data_block,
                data_blocks.clone().len()
            ),
            0,
        );
    }
    base::log("Data blocks encrypted", 0);

    base::log("Encoding encrypted data...", 2);
    let mut finalized_data_blocks: Vec<String> = Vec::new();
    let mut current_encrypted_data_block: u128 = 0;
    for encrypted_data_block in encrypted_data_blocks.clone() {
        current_encrypted_data_block += 1;
        let encoded_encrypted_data_block = b64.encode(encrypted_data_block);
        finalized_data_blocks.push(encoded_encrypted_data_block);
        base::log(
            &format!(
                "Encoded block {}/{}",
                current_encrypted_data_block,
                encrypted_data_blocks.clone().len()
            ),
            0,
        );
    }

    finalized_data_blocks
}

pub fn decrypt(encrypted_data_blocks: Vec<String>, passphrase: String) -> Vec<u8> {
    let mut decoded_encrypted_data_blocks: Vec<Vec<u8>> = Vec::new();
    for encrypted_data_block in encrypted_data_blocks.clone() {
        let decoded_encrypted_data_block = b64.decode(encrypted_data_block).unwrap();
        decoded_encrypted_data_blocks.push(decoded_encrypted_data_block);
    }

    let aes_256_key = decoded_encrypted_data_blocks[0].clone();
    let aes_256_key = rsa::decrypt(aes_256_key, passphrase.clone());
    let aes_256_iv = decoded_encrypted_data_blocks[1].clone();
    let aes_256_iv = rsa::decrypt(aes_256_iv, passphrase.clone());

    let key = aes_256::Key {
        key: aes_256_key,
        iv: aes_256_iv,
    };

    let decoded_encrypted_data_blocks = decoded_encrypted_data_blocks[3..].to_vec();
    let mut decrypted_data: Vec<u8> = Vec::new();
    let mut current_decoded_encrypted_data_block: u128 = 0;
    for decoded_encrypted_data_block in decoded_encrypted_data_blocks.clone() {
        current_decoded_encrypted_data_block += 1;
        let decrypted_data_block = aes_256::decrypt(decoded_encrypted_data_block, &key);
        decrypted_data.extend_from_slice(decrypted_data_block.as_slice());
        base::log(
            &format!(
                "Decrypted block {}/{}",
                current_decoded_encrypted_data_block,
                decoded_encrypted_data_blocks.clone().len()
            ),
            0,
        );
    }

    decrypted_data
}

fn split_into_blocks(data: Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    let mut pos = 0;
    while pos < data.len() {
        let end = std::cmp::min(pos + block_size, data.len());
        let block = &data[pos..end];
        blocks.push(block.to_vec());
        pos = end;
    }
    blocks
}

pub fn correct_passphrase(passphrase: &str) -> bool {
    base::log("Getting private key...", 2);
    let private_key = base::filesystem::cat(&base::filesystem::new_path("base-keys/sys-key"));

    base::log("Checking passphrase...", 2);
    match Rsa::private_key_from_pem_passphrase(private_key.as_bytes(), passphrase.as_bytes()) {
        Ok(_) => {
            base::log("Passphrase is correct", 0);
            true
        }
        Err(_) => {
            base::log("Passphrase is incorrect", 3);
            false
        }
    }
}
