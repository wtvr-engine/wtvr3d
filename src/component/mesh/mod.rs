//! Representation of a mesh in a scene

pub mod mesh_data;

use crate::renderer::buffer::Buffer;
use crate::renderer::material::MaterialInstance;
pub use mesh_data::MeshData;
use web_sys::WebGlRenderingContext;

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh<'a> {
    /// `MeshData` in use for this mesh, containing the vertex data.
    data: MeshData,

    /// `MaterialInstance` to use to render this mesh in the 3d scene
    pub material: MaterialInstance<'a>,
}

impl<'a> Mesh<'a> {
    /// Constructor. Uses a `MeshData` instance and a `MaterialInstance`
    pub fn new(data: MeshData, material: MaterialInstance<'a>) -> Mesh<'a> {
        Mesh {
            data: data,
            material: material,
        }
    }

    /// Returns a slice of the associated `WebGlProgram` buffers
    pub fn get_buffers(&self) -> &[Buffer] {
        self.data.get_buffers()
    }

    /// Returns the number of vertices in this `Mesh`e's `Buffer`s.
    pub fn get_vertex_count(&self) -> i32 {
        self.data.get_vertex_count()
    }

    /// Lookup for this Mesh attributes locations, going though every underlying buffer.  
    /// Should be done at initialization time before any rendering starts.
    pub fn lookup_locations(&mut self, context: &WebGlRenderingContext) -> () {
        self.data
            .lookup_locations(context, self.material.get_parent().borrow().get_program());
        self.material.lookup_locations(context);
    }
}
