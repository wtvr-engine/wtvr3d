//! # mesh
//! Representation of a mesh in a scene

pub mod mesh_data;

use crate::renderer::buffer::Buffer;
use crate::renderer::material::MaterialInstance;
pub use mesh_data::MeshData;
use web_sys::WebGlRenderingContext;

pub struct Mesh<'a> {
    data: MeshData,
    pub material: MaterialInstance<'a>,
}

impl<'a> Mesh<'a> {
    pub fn new(data: MeshData, material: MaterialInstance<'a>) -> Mesh<'a> {
        Mesh {
            data: data,
            material: material,
        }
    }
    pub fn get_buffers(&self) -> &[Buffer] {
        self.data.get_buffers()
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.data.get_vertex_count()
    }

    pub fn lookup_locations(&mut self, context: &WebGlRenderingContext) -> () {
        self.data
            .lookup_locations(context, self.material.get_parent().borrow().get_program());
        self.material.lookup_locations(context);
    }
}
