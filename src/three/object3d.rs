use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    #[derive(Debug)]
    pub type Object3D;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Object3D) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &Object3D, s: String);

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &Object3D) -> Vec<JsValue>;

}
