use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Vector3;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3;
    #[wasm_bindgen(method)]
    pub fn set(this: &Vector3, x: f32, y: f32, z: f32);
    #[wasm_bindgen(method)]
    pub fn add(this: &Vector3, v: Vector3);

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f32;
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f32;
    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, x: f32);
    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, y: f32);
    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, z: f32);


}
