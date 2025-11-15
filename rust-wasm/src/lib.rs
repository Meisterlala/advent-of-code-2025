use wasm_bindgen::prelude::*;
// dsf

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! from Rust+WASM ???sdfsdf?", name)
}
