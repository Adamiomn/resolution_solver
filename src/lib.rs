mod resolution;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_input(input: &str) -> bool {
    resolution::is_input_valid(input)
}

#[wasm_bindgen]
pub fn calculate_resolution(input: &str) -> String {
    match resolution::ClauseGraph::new(input) {
        Ok(clause_graph) => clause_graph.get_solution(),
        Err(message) => message,
    }
}
