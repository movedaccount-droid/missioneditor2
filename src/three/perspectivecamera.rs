use crate::three::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type PerspectiveCamera;
    #[wasm_bindgen(constructor)]
    pub fn new(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;
    #[wasm_bindgen(method, getter = position)]
    pub fn position(this: &PerspectiveCamera) -> Vector3;

}
