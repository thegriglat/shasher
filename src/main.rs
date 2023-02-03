use base64::{engine::general_purpose, Engine};

fn main() {
    let key = "1234567890111111";
    let plaintext = "Hello, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaworldasdasdasdasdasdasdasdsdasdfasasdaf,jkhbasdf,jhasbdfjhbasd,fjb!";
    let algo = pgp::crypto::SymmetricKeyAlgorithm::Twofish;
    let result = algo.encrypt_protected(key.as_bytes(), plaintext.as_bytes());
    match result {
        Ok(v) => {
            // base64-d
            let engine = general_purpose::STANDARD;
            let base64d = engine.encode(v);
            println!("{}", base64d);
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
