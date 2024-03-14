use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type BoxGeometry;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> BoxGeometry;

}
