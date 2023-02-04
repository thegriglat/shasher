use std::{fs, io::Write, process::exit};

use base64::{engine::general_purpose, Engine};
use clap::Parser;
use pgp::crypto::SymmetricKeyAlgorithm;
use sha2::{Digest, Sha256};
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// secret key
    #[arg(short, long, value_name = "key", required = true)]
    key: String,

    /// input file name
    #[arg(short, long, value_name = "input", required = true)]
    input: String,

    /// output file name
    #[arg(short, long, value_name = "output", required = false)]
    output: Option<String>,

    /// decrypt
    #[arg(short, long, default_value_t = false)]
    decrypt: bool,
}

fn get_file_content(filename: String) -> Vec<u8> {
    match fs::read(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Cannot open file!\n{}", e);
            exit(1);
        }
    }
}

fn prepare_key(key: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    hash.to_vec()
}

fn crypto_algorithm() -> SymmetricKeyAlgorithm {
    pgp::crypto::SymmetricKeyAlgorithm::AES256
}

fn decrypt(secret_key: &[u8], content: &[u8]) -> Vec<u8> {
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

fn encrypt(secret_key: &[u8], content: &[u8]) -> Vec<u8> {
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

fn main() {
    let args = Args::parse();

    let secret_key = prepare_key(args.key);
    let content = get_file_content(args.input);
    let output = match args.decrypt {
        true => decrypt(&secret_key, &content),
        false => encrypt(&secret_key, &content),
    };
    match args.output {
        Some(output_filename) => fs::write(output_filename, output).expect("Cannot write to file"),
        None => std::io::stdout()
            .write_all(&output)
            .expect("Cannot write to stdout!"),
    }
}
