//! Interface and implementations for managing WebGL Buffers and Attributes.

use crate::error::Error;
use js_sys::{Float32Array, Uint16Array};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

#[derive(Serialize, Deserialize)]
pub struct Buffer {
    /// Attribute name for the current Buffer.
    attribute_name: String,

    /// Buffer reference;
    #[serde(skip)]
    value: Option<Rc<WebGlBuffer>>,

    /// Index Buffer reference;
    #[serde(skip)]
    indexes: Option<Rc<WebGlBuffer>>,

    /// Actual buffer data
    data: Option<Vec<f32>>,

    /// Size of one vector in the data
    data_vector_size: usize,

    /// Indexes data
    indexes_data: Option<Vec<u16>>,

    /// Numeric type (automatically set); can be Float32, Int16, and UInt8.
    number_type: u32,

    /// Custom stride to be used when setting the attribute pointer
    pub stride: i32,

    /// Offset in the given buffer for the attribute pointer.
    pub offset: i32,
}

impl Buffer {
    pub fn construct(&mut self, context: &WebGlRenderingContext) -> Result<(), Error> {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));
        match &self.data {
            Some(value) => {
                unsafe {
                    let float_array = Float32Array::view(value.as_slice());
                    context.buffer_data_with_array_buffer_view(
                        WebGlRenderingContext::ARRAY_BUFFER,
                        &float_array,
                        WebGlRenderingContext::STATIC_DRAW,
                    );
                }

                if let Some(indexes_array) = &self.indexes_data {
                    if indexes_array.len() > 0 {
                        let gl_index_buffer = context.create_buffer().unwrap();
                        context.bind_buffer(
                            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                            Some(&gl_index_buffer),
                        );

                        unsafe {
                            let uint_array = Uint16Array::view(indexes_array.as_slice());
                            context.buffer_data_with_array_buffer_view(
                                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                                &uint_array,
                                WebGlRenderingContext::STATIC_DRAW,
                            );
                        }
                        self.indexes = Some(Rc::new(gl_index_buffer));
                    }
                }
                Ok(())
            }
            None => Err(Error::MisingData),
        }
    }

    /// Returns the attribute name for this buffer
    pub fn get_attribute_name(&self) -> &str {
        self.attribute_name.as_str()
    }

    /// Enables and sets the attribute pointer at the context level.  
    /// Meant to be called just before rendering.
    pub fn enable_and_bind_attribute(
        &self,
        context: &WebGlRenderingContext,
        location: i32,
    ) -> Result<(), Error> {
        match &self.value {
            Some(val) => {
                context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(val.as_ref()));
                if let Some(index_buffer) = &self.indexes {
                    context.bind_buffer(
                        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                        Some(&index_buffer),
                    );
                }
                let loc = location as u32;
                if location != -1 {
                    context.enable_vertex_attrib_array(loc);
                    context.vertex_attrib_pointer_with_i32(
                        loc,
                        self.data_vector_size as i32,
                        self.number_type,
                        false,
                        self.stride,
                        self.offset,
                    );
                }
                Ok(())
            }
            None => Err(Error::UnconstructedValue),
        }
    }
}
