//! Native and browser capabilities behind one Bevy scene contract.
#[cfg(not(target_arch = "wasm32"))]
#[path = "platform/native.rs"]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
mod web {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = tangentPlatform)]
        pub fn clock_us() -> f64;
        #[wasm_bindgen(js_namespace = tangentPlatform)]
        pub fn store_import(bytes: &[u8], metadata: &str);
        #[wasm_bindgen(js_namespace = tangentPlatform)]
        pub fn pull() -> String;
        #[wasm_bindgen(js_namespace = tangentPlatform)]
        pub fn action(name: &str, data: &str);
        #[wasm_bindgen(js_namespace = tangentPlatform)]
        pub fn report(data: &str);
    }
}
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn native_window(mut windows: bevy::prelude::Query<&mut bevy::window::Window>) {
    use bevy::window::{MonitorSelection, WindowMode};
    if !take_fullscreen_request() {
        return;
    }
    if let Ok(mut window) = windows.single_mut() {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            _ => WindowMode::Windowed,
        };
    }
}
