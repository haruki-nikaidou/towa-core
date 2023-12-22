use wasm_bindgen::prelude::*;
use towa_core;

#[wasm_bindgen]
pub fn encrypt(plain_text: String, key: String, dial: String) {
    towa_core::decrypt(plain_text, key, dial)
}

#[wasm_bindgen]
pub fn decrypt(cipher_text: String, key: String, dial: String) {
    towa_core::decrypt(cipher_text, key, dial)
}