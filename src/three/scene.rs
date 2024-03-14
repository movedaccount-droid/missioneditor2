use crate::three::Mesh;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Scene;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;
    #[wasm_bindgen(method)]
    pub fn add(this: &Scene, mesh: &Mesh);

}
