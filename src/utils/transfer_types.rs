use nalgebra::{Point3, Vector3};
/// Defines a few transfer types to facilitate communciation between JS world and WASM world.
use wasm_bindgen::prelude::*;

/// Simple transfer type for Vector3 since it is not `wasm-bindgen` compatible.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Vector3Data {
    /// x coordinate
    pub x: f32,

    /// y coordinate
    pub y: f32,

    /// z coordinate
    pub z: f32,
}

#[wasm_bindgen]
impl Vector3Data {
    /// Constructor: creates a new VectorData from 3 coordinates.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3Data {
        Vector3Data { x: x, y: y, z: z }
    }
}

impl Vector3Data {
    /// Quick conversion to `nalgebra`'s Point3
    pub fn to_point3(&self) -> Point3<f32> {
        Point3::new(self.x, self.y, self.z)
    }

    /// Quick conversion to `nalgebra`'s Point3
    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}
