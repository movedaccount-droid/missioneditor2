use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Vector3;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3;
    #[wasm_bindgen(method)]
    pub fn set(this: &Vector3, x: f32, y: f32, z: f32);
    #[wasm_bindgen(method)]
    pub fn add(this: &Vector3, v: Vector3);

}
