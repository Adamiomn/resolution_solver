use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_input(input: &str) -> bool {
    return input.len() > 5;
}

#[wasm_bindgen]
pub fn calculate_resolution(input: &str) -> String {
    if input.len() > 10 {
        return "Length greater than 10!".to_owned();
    }
    "Length smaller or equal to 10!".to_owned()
}
