//! Representation of mesh data with its vertices and all buffer data.

use crate::renderer::Buffer;
use serde::{Deserialize,Serialize};

/// Mesh data as the union of its `Buffers` and the number of vertices in the mesh
#[derive(Serialize,Deserialize)]
pub struct Mesh {

    /// Vector of the buffers associated with this mesh: vertex positions, weights, etc.
    buffers: Vec<Buffer>,

    /// Indices array referencing each triangle for the indexed buffers
    vertex_count: i32,

    /// Location lookup state to avoid doing it each frame once it has been done once.
    #[serde(skip)]
    lookup_done: bool,
}