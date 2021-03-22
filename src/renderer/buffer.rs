//! Interface and implementations for managing WebGL Buffers and Attributes.

use web_sys::WebGlBuffer;
use crate::renderer::value::RendererValue;
use std::rc::Rc;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct Buffer {
    /// Attribute name for the current Buffer.
    attribute_name: String,

    /// Buffer reference; several `Buffer` objects can use the same `WebGlBuffer`
    #[serde(skip)]
    value: Option<Rc<WebGlBuffer>>,

    /// Index Buffer reference; several `Buffer` objects can use the same index `WebGlBuffer`
    #[serde(skip)]
    indexes: Option<Rc<WebGlBuffer>>,

    /// Actual buffer data
    data : Option<RendererValue>,

    /// Indexes data
    indexes_data : Option<RendererValue>,

    /// Numeric type (automatically set); can be Float32, Int16, and UInt8.
    number_type: u32,

    /// Custom stride to be used when setting the attribute pointer
    pub stride: i32,

    /// Offset in the given buffer for the attribute pointer.
    pub offset: i32,
}