use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Vector2;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vector2;
    #[wasm_bindgen(method)]
    pub fn set(this: &Vector2, x: f64, y: f64);
    #[wasm_bindgen(method)]
    pub fn add(this: &Vector2, v: Vector2);

}
