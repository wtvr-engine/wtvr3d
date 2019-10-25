//! # MeshData
//!
//! The internal representation of a mesh with its geometry and various properties

use crate::renderer::buffer::Buffer;
use std::vec::Vec;
use web_sys::{WebGlProgram, WebGlRenderingContext};

pub struct MeshData {
    buffers: Vec<Buffer>,
    vertex_count: i32,
}

impl MeshData {
    pub fn new(vertex_count: i32) -> MeshData {
        MeshData {
            buffers: Vec::new(),
            vertex_count: vertex_count,
        }
    }

    pub fn push_buffer(&mut self, buffer: Buffer) -> () {
        self.buffers.push(buffer);
    }

    pub fn get_buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

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
