use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Color;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Color;
    #[wasm_bindgen(method, js_name = setRGB)]
    pub fn set_rgb(this: &Color, r: f32, g: f32, b: f32);

}
