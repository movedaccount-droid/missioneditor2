use crate::three::{BoxGeometry, Euler, MeshBasicMaterial, Vector3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Mesh;
    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &BoxGeometry, material: &MeshBasicMaterial) -> Mesh;
    #[wasm_bindgen(method, getter)]
    pub fn position(geometry: &Mesh) -> Vector3;
    #[wasm_bindgen(method, getter)]
    pub fn rotation(geometry: &Mesh) -> Euler;

}
