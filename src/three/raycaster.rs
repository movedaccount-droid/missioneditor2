use crate::three::{PerspectiveCamera, Vector2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Raycaster;
    #[wasm_bindgen(constructor)]
    pub fn new() -> Raycaster;
    #[wasm_bindgen(method, js_name = setFromCamera)]
    pub fn set_from_camera(
        this: &Raycaster,
        normalized_position: &Vector2,
        camera: &PerspectiveCamera,
    );
    #[wasm_bindgen(method, js_name = intersectObjects)]
    pub fn intersect_objects(
        this: &Raycaster,
        objects: Vec<JsValue>,
        recursive: bool,
    ) -> Vec<JsValue>;

}
