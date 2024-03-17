use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type ImageLoader;
    #[wasm_bindgen(constructor)]
    pub fn new() -> ImageLoader;
    #[wasm_bindgen(method)]
    pub fn load(this: &ImageLoader, url: &str, on_load: &dyn Fn(HtmlImageElement), on_progress: (), on_error: &dyn Fn());

}
