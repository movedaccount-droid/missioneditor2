use crate::three::{PerspectiveCamera, Scene};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type WebGLRenderer;
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLRenderer;
    #[wasm_bindgen(method)]
    pub fn render(this: &WebGLRenderer, scene: &Scene, camera: &PerspectiveCamera);
    #[wasm_bindgen(method, getter = domElement)]
    pub fn dom_element(this: &WebGLRenderer) -> web_sys::HtmlCanvasElement;
    #[wasm_bindgen(method, js_name = setSize)]
    pub fn set_size(this: &WebGLRenderer, width: u32, height: u32, update_style: bool);

}
