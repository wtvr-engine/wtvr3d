//! Representation of mesh data with its vertices and all buffer data.

use crate::renderer::buffer::Buffer;
use crate::renderer::Material;
use web_sys::WebGlRenderingContext;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

/// Mesh data as the union of its `Buffers` and the number of vertices in the mesh
pub struct MeshData {
    /// Unique identifier for this MeshData
    id: String,

    /// Vector of the buffers associated with this mesh: vertex positions, weights, etc.
    buffers: Vec<Buffer>,

    /// Number of vertices in the mesh data.  
    /// The number is not the actual number of vertices in the mesh but rather the number
    /// of vertices in its vertex buffers, with data duplication.
    ///
    /// Ex: A cube has 6 faces, made up of two triangles, and each triangle is 3 vertices
    /// which makes 6 * 2 * 3 = 36 total vertices.
    vertex_count: i32,
}

impl MeshData {
    /// Constructor. The `vertex count` must be the number of vertices in the buffer as specified
    /// on the `Self.vertex_count` property, including duplicates.
    pub fn new(id: String, vertex_count: i32) -> MeshData {
        MeshData {
            id: id,
            buffers: Vec::new(),
            vertex_count: vertex_count,
        }
    }

    /// Add a buffer to this `MeshData`
    pub fn push_buffer(&mut self, buffer: Buffer) -> () {
        self.buffers.push(buffer);
    }

    /// Returns a slice of the available buffers
    pub fn get_buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    /// Returns the number of vertices for this `MeshData`'s Buffers.
    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// Getter for `id`
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Function to lookup the locations for this meshdata;
    pub fn lookup_locations(&self,context: &WebGlRenderingContext, material : Rc<RefCell<Material>>) -> () {
        for buffer in &self.buffers {
            material.borrow_mut().register_new_attribute_location(context, buffer.get_attribute_name())
        }
    }
}
