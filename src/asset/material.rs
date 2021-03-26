//! Material representation in wtvr3d, given a WebGlRenderingContext
//!
//! Materials are responsible of compiling and linking shaders as well as
//! managing WebGlPrograms and their uniforms
//!
//! `Material` represents the WebGlProgram itself and its global uniforms,
//! which can be overriden per-instance.

use crate::renderer::Uniform;
use serde::{Deserialize, Serialize};
use web_sys::WebGlProgram;

#[derive(Serialize, Deserialize)]
pub struct Material {
    /// WebGlProgram for this Material. Computed from vertex and fragment shader at creation time.
    #[serde(skip)]
    program: Option<WebGlProgram>,

    /// if `true`, this Material is opaque (`true` by default), for rendering purposes.
    opaque: bool,

    /// if `true` this material is lit and needs to be recompiled if the number of lights changes
    lit: bool,

    /// Vertex shader text for this material, stored in memory for live re-compilation
    vertex_shader: String,

    /// Fragment shader text for this material, stored in memory for live re-compilation
    fragment_shader: String,

    /// Buffers configuration, with common buffer names and locations.
    #[serde(skip)]
    attribute_locations: Vec<(String, i32)>,

    /// Uniforms shared accross all `MaterialInstance`s sharing this parent material.  
    /// Can be overriden in `MaterialInstance` uniforms if needed.
    shared_uniforms: Vec<Uniform>,

    /// Location lookup state to avoid doing it each frame once it has been done once.
    #[serde(skip)]
    lookup_done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UniformOverrides(Vec<Uniform>);
