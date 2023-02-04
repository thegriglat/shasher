use clap::Parser;
use shasher::{decrypt, encrypt};
use std::{fs, io::Write, process::exit};

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

fn main() {
    let args = Args::parse();
    let content = get_file_content(args.input);

    let output = match args.decrypt {
        true => decrypt(args.key, &content),
        false => encrypt(args.key, &content),
    };
    match args.output {
        Some(output_filename) => fs::write(output_filename, output).expect("Cannot write to file"),
        None => std::io::stdout()
            .write_all(&output)
            .expect("Cannot write to stdout!"),
    }
}
