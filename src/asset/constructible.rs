//! Constructible Trait definition.
//!
//! A Constructible is a type that can be constructed from raw data.
//!
//! For example, a mesh should define buffers and discard the raw data.
//! A Material should compile its shaders and discard the text version.

use web_sys::WebGl2RenderingContext;

use crate::error::W3DError;

/// Trait for everything that needs to be constructed before being used
///
/// For example, a mesh should define buffers and discard the raw data.
/// A Material should compile its shaders and discard the text version.
pub trait Constructible {
    /// Constructs the object so it is usable in a given WebGLContext
    fn construct(&mut self, context: &WebGl2RenderingContext) -> Result<(), W3DError>;

    /// Has the object been constructed yet ?
    fn is_constructed(&self) -> bool;

    /// Free constructed resources if not needed.
    fn deconstruct(&mut self, context: &WebGl2RenderingContext);

    /// Free raw, unconstruced data if not needed anymore.
    fn clean(&mut self);
}
