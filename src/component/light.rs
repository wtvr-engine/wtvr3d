//! Light components for lighting the scene

use nalgebra::{Vector3};
use specs::{HashMapStorage,Component};

/// Directional lights. Does not depend on position and lights the scene in an uniform way
#[derive(Clone)]
pub struct Light {
    pub color : Vector3<f32>,
    pub intensity : f32,
    pub attenuation : f32,
}

#[derive(Clone)]
pub struct Direction(pub Vector3<f32>);

#[derive(Clone)]
pub struct Cone {
    pub blend : f32,
    pub angle : f32,
}



impl Component for Light {
    type Storage = HashMapStorage<Light>;
}

impl Component for Direction {
    type Storage = HashMapStorage<Direction>;
}

impl Component for Cone {
    type Storage = HashMapStorage<Cone>;
}