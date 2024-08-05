use {
    crate::base,
    crate::base::filesystem,
    base64::{prelude::BASE64_STANDARD as b64, Engine},
    openssl::{
        hash::MessageDigest,
        pkey::{PKey, Public},
        rsa::{Padding, Rsa},
        sign::Signer,
        symm::Cipher,
    },
};

const BITS: u32 = 2048;

pub fn xor(text: String, key: String) -> String {
    let mut encrypted_text = String::new();

    for (i, c) in text.chars().enumerate() {
        encrypted_text.push(char::from(
            c as u8 ^ key.chars().nth(i % key.len()).unwrap_or(' ') as u8,
        ));
    }

    encrypted_text
}

pub fn gen_keys(passphrase: String) -> (String, String) {
    let rsa = Rsa::generate(BITS).unwrap();

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

pub fn encrypt(data: Vec<u8>, public_key: String) -> String {
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

    let encrypted_data = &buffer[..size];

    b64.encode(encrypted_data)
}

pub fn decrypt(data: String, passphrase: String) -> String {
    let rsa = Rsa::private_key_from_pem_passphrase(
        filesystem::cat(&filesystem::new_path("base-keys/sys-key")).as_bytes(),
        passphrase.as_bytes(),
    )
    .expect("Can't convert private key from pem to object");

    let encrypted_data: Vec<u8> = match b64.decode(data) {
        Ok(data) => data,
        Err(err) => {
            base::log(&format!("Can't decode data: {}", err), 1);
            vec![]
        }
    };
    let mut buffer = vec![0; rsa.size() as usize];
    let size = match rsa.private_decrypt(&encrypted_data, &mut buffer, openssl::rsa::Padding::PKCS1)
    {
        Ok(size) => size,
        Err(err) => {
            base::log(&format!("Can't encrypt data using public key: {}", err), 1);
            0
        }
    };

    let decrypted_data: &[u8] = &buffer[..size];

    match String::from_utf8(decrypted_data.to_vec()) {
        Ok(string) => string,
        Err(err) => {
            base::log(
                &format!("Can't convert decrypted data to string: {}", err),
                1,
            );
            String::new()
        }
    }
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
