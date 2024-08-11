use {
    crate::base::{self, filesystem},
    base64::{prelude::BASE64_STANDARD as b64, Engine},
    openssl::{
        hash::MessageDigest,
        pkey::{PKey, Private, Public},
        rsa::{Padding, Rsa},
        sign::Signer,
        symm::Cipher,
    },
};

pub fn gen_keys(passphrase: String, bits: u32) -> (String, String) {
    let rsa: Rsa<Private> = match Rsa::generate(bits) {
        Ok(object) => object,
        Err(err) => {
            base::log(&format!("Can't generate keys: {}", err), 1);
            Rsa::generate(bits).unwrap()
        }
    };

    let private_key: Vec<u8> =
        match rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes()) {
            Ok(object) => object,
            Err(err) => {
                base::log(&format!("Can't convert private key to pem: {}", err), 1);
                Vec::new()
            }
        };
    let public_key: Vec<u8> = match rsa.public_key_to_pem_pkcs1() {
        Ok(object) => object,
        Err(err) => {
            base::log(&format!("Can't convert public key to pem: {}", err), 1);
            Vec::new()
        }
    };
    let private_key: String = match String::from_utf8(private_key) {
        Ok(object) => object,
        Err(err) => {
            base::log(
                &format!("Can't convert private key from Vec<u8> to String: {}", err),
                1,
            );
            String::new()
        }
    };
    let public_key: String = match String::from_utf8(public_key) {
        Ok(object) => object,
        Err(err) => {
            base::log(
                &format!("Can't convert public key from Vec<u8> to String: {}", err),
                1,
            );
            String::new()
        }
    };

    (private_key, public_key)
}

pub fn sign(data: String, passphrase: String) -> String {
    let rsa: Rsa<Private> = match Rsa::private_key_from_pem_passphrase(
        filesystem::cat(&filesystem::new_path("base-keys/sys-key")).as_bytes(),
        passphrase.as_bytes(),
    ) {
        Ok(object) => object,
        Err(err) => {
            base::log(
                &format!("Can't convert private key from pem to object: {}", err),
                1,
            );
            Rsa::private_key_from_pem_passphrase(
                filesystem::cat(&filesystem::new_path("base-keys/sys-key")).as_bytes(),
                passphrase.as_bytes(),
            )
            .unwrap()
        }
    };

    let private_key = match PKey::from_rsa(rsa.clone()) {
        Ok(object) => object,
        Err(err) => {
            base::log(&format!("Can't convert private key to object: {}", err), 1);
            PKey::from_rsa(rsa).unwrap()
        }
    };

    let mut signer = match Signer::new(MessageDigest::sha256(), &private_key) {
        Ok(object) => object,
        Err(err) => {
            base::log(&format!("Can't create signer: {}", err), 1);
            Signer::new(MessageDigest::sha256(), &private_key).unwrap()
        }
    };
    match signer.set_rsa_padding(Padding::PKCS1) {
        Ok(()) => (),
        Err(err) => base::log(&format!("Can't set padding: {}", err), 1),
    };
    match signer.update(data.as_bytes()) {
        Ok(()) => (),
        Err(err) => base::log(&format!("Can't update signer: {}", err), 1),
    };

    let signature = match signer.sign_to_vec() {
        Ok(object) => object,
        Err(err) => {
            base::log(&format!("Can't sign data: {}", err), 1);
            Vec::new()
        }
    };

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
    let rsa = match Rsa::private_key_from_pem_passphrase(
        filesystem::cat(&filesystem::new_path("base-keys/my-key")).as_bytes(),
        passphrase.as_bytes(),
    ) {
        Ok(object) => object,
        Err(err) => {
            base::log(
                &format!("Can't convert private key from pem to object: {}", err),
                1,
            );
            Rsa::private_key_from_pem_passphrase(
                filesystem::cat(&filesystem::new_path("base-keys/my-key")).as_bytes(),
                passphrase.as_bytes(),
            )
            .unwrap()
        }
    };

    let mut buffer = vec![0; rsa.size() as usize];
    let size = match rsa.private_decrypt(&encrypted_data, &mut buffer, openssl::rsa::Padding::PKCS1)
    {
        Ok(size) => size,
        Err(err) => {
            base::log(&format!("Can't decrypt data using private key: {}", err), 1);
            0
        }
    };

    buffer[..size].to_vec()
}
