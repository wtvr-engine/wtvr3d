//! Representation of a mesh in a scene

use specs::{Component, VecStorage};

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh {
    material: String,
    material_instance: String,
    mesh_data: String,
}

impl Mesh {
    /// Constructor. Uses a `MeshData` id and a `MaterialInstance` id.
    pub fn new(mesh_data_id: &str, material_instance_id: &str, material_id: &str) -> Mesh {
        Mesh {
            mesh_data: mesh_data_id.to_owned(),
            material: material_id.to_owned(),
            material_instance: material_instance_id.to_owned(),
        }
    }
    /// Getter for material
    pub fn get_material_instance_id(&self) -> &str {
        &self.material_instance
    }

    /// Getter for material
    pub fn get_material_id(&self) -> &str {
        &self.material
    }

    /// Getter for mesh_data
    pub fn get_mesh_data_id(&self) -> &str {
        &self.mesh_data
    }
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}
