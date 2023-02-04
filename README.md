# shasher
Simple AES 256 encrypt/decrypt CLI utility


## Usage

```bash
Usage: shasher [OPTIONS] --key <key> --input <input>

Options:
  -k, --key <key>        secret key
  -i, --input <input>    input file name
  -o, --output <output>  output file name
  -d, --decrypt          decrypt
  -h, --help             Print help
```

## Run in develop mode
```bash
cargo run -- -k hello -i file.bin -o file.enc
# or
cargo run --release -- -k hello -i file.bin -o file.enc
```

## build

```bash
cargo build --release
```
