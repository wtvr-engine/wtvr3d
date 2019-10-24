//! # Mesh
//! 
//! The internal representation of a mesh with its geometry and various properties

use std::vec::Vec;

pub struct StaticMesh {
    pub geometry : Vec<f32>,
}

pub struct SkinnedMesh {
    pub static_mesh : StaticMesh,
    pub weights : Vec<f32>,
}

pub trait Mesh {
    fn geometry(&self) -> &[f32];
}

impl Mesh for StaticMesh {
    fn geometry(&self) -> &[f32]{
        self.geometry.as_slice()
    }
}

impl Mesh for SkinnedMesh {
    fn geometry(&self) -> &[f32] {
        self.static_mesh.geometry()
    }
}