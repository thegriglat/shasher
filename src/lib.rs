use std::process::exit;

use base64::{engine::general_purpose, Engine};
use pgp::crypto::SymmetricKeyAlgorithm;
use sha2::{Digest, Sha256};

fn prepare_key(key: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    hash.to_vec()
}

fn crypto_algorithm() -> SymmetricKeyAlgorithm {
    pgp::crypto::SymmetricKeyAlgorithm::AES256
}

pub fn decrypt(key: String, content: &[u8]) -> Vec<u8> {
    let secret_key = prepare_key(key);
    let engine = general_purpose::STANDARD;
    let mut content_bytes = match engine.decode(content) {
        Ok(val) => val,
        Err(e) => {
            println!("Decode base64 error: {}", e);
            exit(1)
        }
    };
    let encrypted_data = match crypto_algorithm().decrypt_protected(&secret_key, &mut content_bytes)
    {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    };
    encrypted_data.to_vec()
}

pub fn encrypt(key: String, content: &[u8]) -> Vec<u8> {
    let secret_key = prepare_key(key);
    let result = crypto_algorithm().encrypt_protected(&secret_key, &content);
    match result {
        Ok(v) => {
            let engine = general_purpose::STANDARD;
            let base64_content = engine.encode(v);
            base64_content.as_bytes().to_vec()
        }
        Err(e) => {
            println!("Encrypt error: {}", e);
            exit(1)
        }
    }
}
