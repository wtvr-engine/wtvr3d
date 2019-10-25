//! # MeshData
//!
//! The internal representation of a mesh with its geometry and various properties

use std::vec::Vec;

pub struct StaticMeshData {
    pub geometry: Vec<f32>,
}

pub struct SkinnedMeshData {
    pub static_mesh: StaticMeshData,
    pub weights: Vec<f32>,
}

pub enum MeshData {
    Static(StaticMeshData),
    Skinned(SkinnedMeshData),
}

impl MeshData {
    pub fn geometry(&self) -> &[f32] {
        match self {
            MeshData::Skinned(m) => m.static_mesh.geometry.as_slice(),
            MeshData::Static(m) => m.geometry.as_slice(),
        }
    }
}
