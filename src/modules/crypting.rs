use {
    crate::base,
    crate::base::filesystem,
    crate::modules::config,
    base64::{prelude::BASE64_STANDARD as b64, Engine},
    openssl::{
        hash::MessageDigest,
        pkey::{PKey, Public},
        rsa::{Padding, Rsa},
        sign::Signer,
        symm::{self, Cipher},
    },
    rand::rngs::OsRng,
    rand::RngCore,
};

pub fn gen_keys(passphrase: String, bits: u32) -> (String, String) {
    let rsa = Rsa::generate(bits).unwrap();

    let private_key: Vec<u8> = rsa
        .private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes())
        .expect("Failed to convert private key to pem and encrypt");
    let public_key: Vec<u8> = rsa
        .public_key_to_pem_pkcs1()
        .expect("Failed to convert public key to pem");
    let private_key: String =
        String::from_utf8(private_key).expect("Can't convert private key from Vec<u8> to String");
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

    let private_key = PKey::from_rsa(rsa).expect("Can't convert private key from object to PKey");

    let mut signer =
        Signer::new(MessageDigest::sha256(), &private_key).expect("Can't create signer");
    signer
        .set_rsa_padding(Padding::PKCS1)
        .expect("Can't set rsa padding");
    signer.update(data.as_bytes()).expect("Can't update data");

    let signature = signer.sign_to_vec().expect("Can't sign data");

    b64.encode(&signature)
}

pub fn encrypt_data(data: &[u8], public_key: String) -> ((String, u128), Vec<String>) {
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

    let mut sym_key = vec![0; 32];
    let mut iv = vec![0; 16];

    OsRng.fill_bytes(&mut sym_key);
    OsRng.fill_bytes(&mut iv);

    let mut encrypted_key = vec![0; rsa.size() as usize];
    let encrypted_key_len = match rsa.public_encrypt(&sym_key, &mut encrypted_key, Padding::PKCS1) {
        Ok(size) => size,
        Err(err) => {
            base::log(&format!("Can't encrypt symmetric key: {}", err), 1);
            0
        }
    };

    base::log("Encrypted symmetric key", 0);

    let mut content_blocks: Vec<Vec<u8>> = Vec::new();

    content_blocks.push(encrypted_key[..encrypted_key_len].to_vec());
    content_blocks[0].extend_from_slice(&iv);

    let data_blocks = split_into_blocks(data.to_vec(), config::BLOCK_BITS as usize);
    let mut encrypted_data_blocks: Vec<Vec<u8>> = vec![];

    for data_block in data_blocks {
        let cipher = Cipher::aes_256_cbc();
        let encrypted_data_block = match symm::encrypt(cipher, &sym_key, Some(&iv), &data_block) {
            Ok(encrypted_data_block) => encrypted_data_block,
            Err(err) => {
                base::log(&format!("Can't encrypt data: {}", err), 1);
                vec![]
            }
        };
        encrypted_data_blocks.push(encrypted_data_block);
    }
    content_blocks.extend_from_slice(&encrypted_data_blocks);

    sym_key.fill(0);
    iv.fill(0);

    let mut finalized_content_blocks: Vec<String> = vec![];
    for content_block in content_blocks {
        let encrypted_content_block = b64.encode(&content_block);
        finalized_content_blocks.push(encrypted_content_block);
    }
    let finalized_content_uuid = base::uuid::generate();

    (
        (
            finalized_content_uuid,
            finalized_content_blocks.len() as u128,
        ),
        finalized_content_blocks,
    )
}

pub fn decrypt_data(encrypted_blocks: Vec<String>, passphrase: String) -> Vec<u8> {
    let private_key = filesystem::bcat(
        filesystem::new_path("base-keys/my-key")
            .display()
            .to_string(),
    );

    base::log("Flag 1", 3);

    let rsa = Rsa::private_key_from_pem_passphrase(&private_key, passphrase.as_bytes())
        .expect("Invalid private key");
    let key_size = rsa.size() as usize;

    base::log("Flag 2", 3);

    let first_block = &encrypted_blocks[0];
    let decoded_first_block = b64
        .decode(first_block)
        .expect("Failed to decode base64 block");

    base::log("Flag 3", 3);

    if decoded_first_block.len() < key_size {
        panic!("Decoded block is smaller than expected RSA key size");
    }

    base::log("Flag 4", 3);

    let (encrypted_key, iv) = decoded_first_block.split_at(key_size);

    base::log("Flag 5", 3);

    let mut sym_key = vec![0_u8; key_size];
    if rsa
        .private_decrypt(encrypted_key, &mut sym_key, Padding::PKCS1)
        .is_err()
    {
        base::log("Decryption failed", 1);
        panic!("Decryption failed");
    }

    base::log("Flag 6", 3);

    let cipher = Cipher::aes_256_cbc();
    let mut decrypted_data: Vec<u8> = Vec::new();

    base::log("Flag 7", 3);

    for block in &encrypted_blocks[1..] {
        let decoded_block = b64.decode(block).expect("Failed to decode base64 block");
        let decrypted_block = match symm::decrypt(cipher, &sym_key, Some(&iv), &decoded_block) {
            Ok(decrypted_block) => decrypted_block,
            Err(err) => {
                eprintln!("Can't decrypt data: {}", err);
                continue;
            }
        };
        decrypted_data.extend_from_slice(&decrypted_block);
    }

    sym_key.fill(0);

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
