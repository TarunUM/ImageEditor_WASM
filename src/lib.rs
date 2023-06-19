use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::{load_from_memory, ImageOutputFormat::Png};

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    let base64_to_vector = decode(encoded_file).unwrap();

    let mut img = load_from_memory(&base64_to_vector).unwrap();

    img = img.fliph();
    img = img.grayscale();

    let mut buffer = Vec::new();
    img.write_to(&mut buffer, Png).unwrap();
    log(&"Image written".into());
    let encoded_image = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_image);
    return data_url;
}