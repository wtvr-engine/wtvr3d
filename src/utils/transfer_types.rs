/// Defines a few transfer types to facilitate communciation between JS world and WASM world.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vector3Data {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
impl Vector3Data {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3Data {
        Vector3Data { x: x, y: y, z: z }
    }
}
