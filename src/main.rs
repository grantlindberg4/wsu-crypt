use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

pub mod convert_types;
pub mod crypt;
pub mod subkey_gen;
pub mod whiten;

#[cfg(test)]
mod test;

const NUM_EXPECTED_ARGS: usize = 2;

/// Displays a message telling the user how to properly run the program
/// Invoked when a command line parsing error has occurred
fn display_error_message() {
    eprintln!("Usage:\n");
    eprintln!("Execute 'cargo run encrypt' to perform encryption");
    eprintln!("Note: 'plaintext.txt' and 'key.txt' must exist under 'input/' for this to work\n");
    eprintln!("Execute 'cargo run decrypt' to perform decryption");
    eprintln!("Note: 'ciphertext.txt' and 'key.txt' must exist under 'input/' for this to work");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != NUM_EXPECTED_ARGS {
        display_error_message();
        process::exit(1);
    }

    match args[1].as_ref() {
        "encrypt" => {
            let mut f = File::open("input/plaintext.txt").expect("File for plaintext not found");
            let mut plaintext = String::new();
            f.read_to_string(&mut plaintext).expect("Issue parsing the file");
            let plaintext = plaintext.into_bytes();

            let mut f = File::open("input/key.txt").expect("File for key not found");
            let mut key = String::new();
            f.read_to_string(&mut key).expect("Issue parsing the file");
            let key = key.into_bytes();

            // let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
            // let plaintext: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

            let ciphertext = crypt::encrypt(&key, &plaintext);
            println!("{:x}", ciphertext);
        },
        "decrypt" => {
            let mut f = File::open("input/ciphertext.txt").expect("File for ciphertext not found");
            let mut ciphertext = String::new();
            f.read_to_string(&mut ciphertext).expect("Issue parsing the file");
            let ciphertext = ciphertext.into_bytes();

            let mut f = File::open("input/key.txt").expect("File for key not found");
            let mut key = String::new();
            f.read_to_string(&mut key).expect("Issue parsing the file");
            let key = key.into_bytes();

            // let key: Vec<u8> = vec![0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89];
            // let ciphertext: Vec<u8> = vec![0x2a, 0xd9, 0xc6, 0xe5, 0xb8, 0xfe, 0x56, 0xfb];

            let plaintext = crypt::decrypt(&key, &ciphertext);

            println!("{:x}", plaintext);
        },
        _ => {
            display_error_message();
            process::exit(1);
        }
    }
}
