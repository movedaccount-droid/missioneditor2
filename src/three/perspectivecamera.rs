use crate::three::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type PerspectiveCamera;
    #[wasm_bindgen(constructor)]
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> PerspectiveCamera;
    #[wasm_bindgen(method, getter = position)]
    pub fn position(this: &PerspectiveCamera) -> Vector3;

}
