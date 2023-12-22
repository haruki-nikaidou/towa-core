use crate::combination_lock::CombinationLock;
use crate::encode::{decode_base64, decode_utf8, encode_base64, encode_utf8};

mod three_fish;
mod pad;
mod combination_lock;
mod get_keys;
mod pow;
mod encode;
mod parse_array;

pub fn encrypt(plain_text: String, key: String, dial: String) {
    let plain_text = decode_utf8(plain_text);
    let key = decode_utf8(key);
    let dial = parse_array::string_to_vec(dial);
    let mut lock = CombinationLock::new(key, dial);
    let cipher_text = lock.encrypt(&plain_text);
    let result = encode_base64(cipher_text);
    println!("Encrypted text:\n {}", result);
}

pub fn decrypt(cipher_text: String, key: String, dial: String) {
    let cipher_text = decode_base64(cipher_text);
    let key = decode_base64(key);
    let dial = parse_array::string_to_vec(dial);
    let mut lock = CombinationLock::new(key, dial);
    let plain_text = lock.decrypt(&cipher_text);
    let result = encode_utf8(plain_text);
    println!("Decrypted text:\n {}", result);
}