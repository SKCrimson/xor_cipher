#![warn(clippy::all, clippy::pedantic)]

use bytes::Bytes;
use std::cmp::Ordering;
use std::io;

fn main() {
    cipher();

    decipher();
}

fn cipher() {
    println!("Enter your string: ");

    let input = read_str();

    println!("Enter your password: ");

    let password = pad_or_trim(input.len(), read_str().as_bytes());

    let result = zip_and_xor(&password, Bytes::from(input.clone()));

    let ciphed_string = String::new()
        + &result
            .iter()
            .map(|byte| format!("{byte:02x} "))
            .collect::<String>();

    println!("Ciphered string: ");
    println!("{ciphed_string}");
}

fn decipher() {
    println!("Enter your ciphered string: ");

    let input = read_str();

    let bytes_data = Bytes::from(
        input
            .split_whitespace()
            .map(|hex| u8::from_str_radix(hex, 16).unwrap())
            .collect::<Vec<u8>>(),
    );

     println!("Enter your password: ");

    let password = pad_or_trim(input.len() / 2, read_str().as_bytes());

    let result = zip_and_xor(&password, bytes_data);

    let deciphered_string = String::from_utf8(result).unwrap();

    println!("Deciphered string: ");
    println!("{deciphered_string}");    
}

fn pad_or_trim(limit: usize, original_password: &[u8]) -> Vec<u8> {
    let password_len = original_password.len();

    match limit.cmp(&password_len) {
        Ordering::Less => original_password.iter().copied().take(limit).collect(),
        Ordering::Equal => original_password.to_vec(),
        Ordering::Greater => {
            let mut as_bytes = original_password.to_vec();

            for i in password_len..limit {
                as_bytes.push(as_bytes[i % password_len]);
            }

            as_bytes
        }
    }
}

fn zip_and_xor(password: &[u8], bytes_data: Bytes) -> Vec<u8> {
    password
        .iter()
        .zip(bytes_data)
        .map(|(&a, b)| a ^ b)
        .collect()
}

fn read_str() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().into()
}
