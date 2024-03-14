use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {

    pub type Euler;
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Euler;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Euler) -> f32;
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Euler) -> f32;
    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Euler) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Euler, x: f32) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Euler, y: f32) -> f32;
    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Euler, z: f32) -> f32;

}
