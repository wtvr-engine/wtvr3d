//! Representation of a mesh in a scene

use specs::{Component, VecStorage};

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh {
    material: String,
    mesh_data: String,
}

impl Mesh {
    /// Constructor. Uses a `MeshData` id and a `MaterialInstance` id.
    pub fn new(mesh_data_id: &str, material_instance_id: &str) -> Mesh {
        Mesh {
            mesh_data: mesh_data_id.to_owned(),
            material: material_instance_id.to_owned(),
        }
    }
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}
