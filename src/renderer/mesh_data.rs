//! Representation of mesh data with its vertices and all buffer data.

use crate::renderer::buffer::Buffer;
use crate::renderer::Material;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use web_sys::WebGlRenderingContext;

/// Mesh data as the union of its `Buffers` and the number of vertices in the mesh
pub struct MeshData {
    /// Unique identifier for this MeshData
    id: String,

    /// Vector of the buffers associated with this mesh: vertex positions, weights, etc.
    buffers: Vec<Buffer>,

    /// Indices array referencing each triangle for the indexed buffers
    vertex_count : i32,

     /// Location lookup state to avoid doing it each frame once it has been done once.
     lookup_done : bool,
}

impl MeshData {
    /// Constructor. The `vertex count` must be the number of vertices in the buffer as specified
    /// on the `Self.vertex_count` property, including duplicates.
    pub fn new(id: String, vertex_count : i32) -> MeshData {
        MeshData {
            id: id,
            buffers: Vec::new(),
            vertex_count : vertex_count,
            lookup_done : false,
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

    pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
        for buffer in &self.buffers {
            if buffer.get_attribute_name() == name {
                return Some(buffer);
            }
        }
        None
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
    pub fn lookup_locations(
        &mut self,
        context: &WebGlRenderingContext,
        material: Rc<RefCell<Material>>,
    ) -> () {
        if self.lookup_done {
            return;
        }
        for buffer in &self.buffers {
            material
                .borrow_mut()
                .register_new_attribute_location(context, buffer.get_attribute_name())
        }
        self.lookup_done = true;
    }
}
