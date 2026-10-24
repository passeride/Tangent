//! Browser capabilities only; no controls, layout, notation or rendering in JS.
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tangentPlatform)]
    pub fn clock_us() -> f64;
    #[wasm_bindgen(js_namespace = tangentPlatform)]
    pub fn pull() -> String;
    #[wasm_bindgen(js_namespace = tangentPlatform)]
    pub fn action(name: &str, data: &str);
    #[wasm_bindgen(js_namespace = tangentPlatform)]
    pub fn report(data: &str);
}
