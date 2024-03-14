use crate::three::PerspectiveCamera;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/examples/jsm/controls/OrbitControls-modified.js")]
extern "C" {

    pub type OrbitControls;
    #[wasm_bindgen(constructor)]
    pub fn new(
        camera: &PerspectiveCamera,
        dom_element: &web_sys::HtmlCanvasElement,
    ) -> OrbitControls;
    #[wasm_bindgen(method)]
    pub fn update(this: &OrbitControls);

}
