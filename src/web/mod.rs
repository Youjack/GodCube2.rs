#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
pub fn search_wasm(config_js: JsValue) -> Result<JsValue, JsValue> {
    let (algo_str, state): (String, Cube2State) = serde_wasm_bindgen::from_value(config_js)?;
    let config = Config {
        algo: match &algo_str[..] {
            "BFS" => AlgoKind::BFS,
            _ => AlgoKind::BFS, // not implemented for IDA*
        },
        initial_node: Cube2::new(state),
    };
    
    let sol = search(config)?;
    Ok(serde_wasm_bindgen::to_value(&sol)?)
}
