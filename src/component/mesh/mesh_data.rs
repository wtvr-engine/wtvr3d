//! Representation of mesh data with its vertices and all buffer data.

use crate::renderer::buffer::Buffer;
use std::vec::Vec;
use web_sys::{WebGlProgram, WebGlRenderingContext};


/// Mesh data as the union of its `Buffers` and the number of vertices in the mesh
pub struct MeshData {

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
    pub fn new(vertex_count: i32) -> MeshData {
        MeshData {
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

    /// Compute this MeshData's buffer locations in the programs.  
    /// Must be done at initialization time, passing a valid context and program.
    pub fn lookup_locations(
        &mut self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        for uniform in &mut self.buffers {
            uniform.lookup_location(context, program);
        }
    }
}
