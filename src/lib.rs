use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::{load_from_memory, ImageOutputFormat::Png};

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    log(&"1 Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"2 Image base64 decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"3 Image decoded".into());

    img = img.fliph();
    img = img.grayscale();
    log(&"4 Grayscale effect applied".into());

    let mut buffer = Vec::new();
    img.write_to(&mut buffer, Png).unwrap();
    log(&"5 New Image writtern".into());

    let encoded_image = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_image);
    return data_url;
}