use std::{fs, io::Write, process::exit};

use base64::{engine::general_purpose, Engine};
use clap::Parser;
use sha2::{Digest, Sha256};
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// secret key
    #[arg(short, long)]
    key: String,

    /// filename
    #[arg(short, long)]
    file: String,

    /// decrypt file
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

fn main() {
    let crypto_algorithm = pgp::crypto::SymmetricKeyAlgorithm::AES256;
    let args = Args::parse();

    let secret_key = prepare_key(args.key);
    let content = get_file_content(args.file);
    match args.decrypt {
        true => {
            let engine = general_purpose::STANDARD;
            let mut content_bytes = match engine.decode(content) {
                Ok(val) => val,
                Err(e) => {
                    println!("Decode base64 error: {}", e);
                    exit(1)
                }
            };
            let encrypted_data = crypto_algorithm
                .decrypt_protected(&secret_key, &mut content_bytes)
                .expect("Cannot decrypt data");
            std::io::stdout()
                .write_all(encrypted_data)
                .expect("stdout error")
        }
        false => {
            let result = crypto_algorithm.encrypt_protected(&secret_key, &content);
            match result {
                Ok(v) => {
                    let engine = general_purpose::STANDARD;
                    let base64_content = engine.encode(v);
                    print!("{}", base64_content);
                }
                Err(e) => {
                    println!("Encrypt error: {}", e);
                    exit(1)
                }
            }
        }
    }
}
