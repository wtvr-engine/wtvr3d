//! # mesh
//! Representation of a mesh in a scene

pub mod mesh_data;

use crate::renderer::material::MaterialInstance;
use mesh_data::MeshData;

pub struct Mesh<'a> {
    data: MeshData,
    pub material: MaterialInstance<'a>,
}
