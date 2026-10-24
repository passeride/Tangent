//! JSON boundary for an import-only browser harness. No device/audio access.
pub fn import_json(bytes: &[u8]) -> Result<String, String> {
    let song = tangent_midi::import(bytes)
        .map_err(|e| serde_json::to_string(&e).expect("import errors serialize"))?;
    serde_json::to_string(&song).map_err(|e| e.to_string())
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn inspect_midi(bytes: &[u8]) -> Result<String, wasm_bindgen::JsValue> {
    import_json(bytes).map_err(|e| wasm_bindgen::JsValue::from_str(&e))
}
#[cfg(test)]
mod tests {
    #[test]
    fn errors_cross_the_boundary_as_structured_json() {
        let error = super::import_json(b"not midi").unwrap_err();
        let error: serde_json::Value = serde_json::from_str(&error).unwrap();
        assert_eq!(error["code"], "InvalidFile");
    }
}
