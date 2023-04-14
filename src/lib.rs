mod resolution;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn try_parse_input(input: &str) -> String {
    match resolution::try_parse_input(input) {
        Ok(_) => "✅".to_owned(),
        Err(message) => format!("❌ {}", message),
    }
}

#[wasm_bindgen]
pub fn calculate_resolution(input: &str) -> String {
    match resolution::ClauseGraph::new(input) {
        Ok(clause_graph) => clause_graph.get_solution(),
        Err(message) => message,
    }
}
