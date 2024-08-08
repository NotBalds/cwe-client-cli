mod aes_256 {
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
}

pub mod rsa {
    use {
        crate::base::{self, filesystem},
        base64::{prelude::BASE64_STANDARD as b64, Engine},
        openssl::{
            hash::MessageDigest,
            pkey::{PKey, Public},
            rsa::{Padding, Rsa},
            sign::Signer,
            symm::Cipher,
        },
    };
    pub fn gen_keys(passphrase: String, bits: u32) -> (String, String) {
        let rsa = Rsa::generate(bits).unwrap();

        let private_key: Vec<u8> = rsa
            .private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes())
            .expect("Failed to convert private key to pem and encrypt");
        let public_key: Vec<u8> = rsa
            .public_key_to_pem_pkcs1()
            .expect("Failed to convert public key to pem");
        let private_key: String = String::from_utf8(private_key)
            .expect("Can't convert private key from Vec<u8> to String");
        let public_key: String =
            String::from_utf8(public_key).expect("Can't convert public key from Vec<u8> to String");

        (private_key, public_key)
    }

    pub fn sign(data: String, passphrase: String) -> String {
        let rsa = Rsa::private_key_from_pem_passphrase(
            filesystem::cat(&filesystem::new_path("base-keys/sys-key")).as_bytes(),
            passphrase.as_bytes(),
        )
        .expect("Can't convert private key from pem to object");

        let private_key =
            PKey::from_rsa(rsa).expect("Can't convert private key from object to PKey");

        let mut signer =
            Signer::new(MessageDigest::sha256(), &private_key).expect("Can't create signer");
        signer
            .set_rsa_padding(Padding::PKCS1)
            .expect("Can't set rsa padding");
        signer.update(data.as_bytes()).expect("Can't update data");

        let signature = signer.sign_to_vec().expect("Can't sign data");

        b64.encode(&signature)
    }

    pub fn encrypt(data: Vec<u8>, public_key: String) -> Vec<u8> {
        let rsa = match Rsa::public_key_from_pem_pkcs1(public_key.as_bytes()) {
            Ok(object) => object,
            Err(err) => {
                base::log(
                    &format!("Can't convert public key from pem to object: {}", err),
                    1,
                );
                let tmp: Rsa<Public> = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
                tmp
            }
        };

        let mut buffer = vec![0; rsa.size() as usize];
        let size = match rsa.public_encrypt(&data, &mut buffer, openssl::rsa::Padding::PKCS1) {
            Ok(size) => size,
            Err(err) => {
                base::log(&format!("Can't encrypt data using public key: {}", err), 1);
                0
            }
        };

        buffer[..size].to_vec()
    }

    pub fn decrypt(encrypted_data: Vec<u8>, passphrase: String) -> Vec<u8> {
        let rsa = Rsa::private_key_from_pem_passphrase(
            filesystem::cat(&filesystem::new_path("base-keys/my-key")).as_bytes(),
            passphrase.as_bytes(),
        )
        .expect("Can't convert private key from pem to object");

        let mut buffer = vec![0; rsa.size() as usize];
        let size =
            match rsa.private_decrypt(&encrypted_data, &mut buffer, openssl::rsa::Padding::PKCS1) {
                Ok(size) => size,
                Err(err) => {
                    base::log(&format!("Can't decrypt data using private key: {}", err), 1);
                    0
                }
            };

        buffer[..size].to_vec()
    }
}

pub mod base {
    use {
        crate::{
            base,
            modules::{
                config,
                crypting::{aes_256, rsa},
            },
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
}
