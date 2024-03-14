use crate::three::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type MeshBasicMaterial;
    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshBasicMaterial;
    #[wasm_bindgen(method, getter = color)]
    pub fn color(this: &MeshBasicMaterial) -> Color;

}
