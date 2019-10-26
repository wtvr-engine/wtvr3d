//! # Buffer
//!
//! Interface and implementations for managing Buffers and Attributes.

use super::shader_data_type::ShaderDataType;
use js_sys::{Float32Array, Int16Array, Uint8Array};
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

/// ## Buffer
///
/// A `Buffer` reprensents information about an attribute and its buffer.  
/// It contains its attribute's name, location, buffer value, as well as
/// its data type and buffer offset.
#[derive(Clone)]
pub struct Buffer {
    /// Attribute name for the current Buffer.
    attribute_name: String,

    /// Automatically computed attribute location for use in WebGL.
    attribute_location: Option<i32>,

    /// Buffer reference; several `Buffer` objects can use the same `WebGlBuffer`
    value: Rc<WebGlBuffer>,

    /// Data type in the shader as defined in `ShaderDataType`
    data_type: ShaderDataType,

    /// Numeric type (automatically set); can be Float32, Int16, and UInt8.
    number_type: u32,

    /// Custom stride to be used when setting the attribute pointer
    pub stride: i32,

    /// Offset in the giver buffer for the attribute pointer.
    pub offset: i32,
}

impl Buffer {
    /// Constructor using a instance of `Float32Array` and an attribute name
    pub fn from_f32_data(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: Float32Array,
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));

        context.buffer_data_with_opt_array_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&data.buffer()),
            WebGlRenderingContext::STATIC_DRAW,
        );

        Buffer {
            attribute_name: String::from(name),
            attribute_location: None,
            value: Rc::new(gl_buffer),
            data_type: data_type,
            stride: 0,
            offset: 0,
            number_type: WebGlRenderingContext::FLOAT,
        }
    }

    /// Constructor using a instance of `Int16Array` and an attribute name
    pub fn from_i16_data(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: Int16Array,
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));

        context.buffer_data_with_opt_array_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&data.buffer()),
            WebGlRenderingContext::STATIC_DRAW,
        );

        Buffer {
            attribute_name: String::from(name),
            attribute_location: None,
            value: Rc::new(gl_buffer),
            data_type: data_type,
            stride: 0,
            offset: 0,
            number_type: WebGlRenderingContext::SHORT,
        }
    }

    /// Constructor using a instance of `Uint8Array` and an attribute name
    pub fn from_u8_data(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: Uint8Array,
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));

        context.buffer_data_with_opt_array_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&data.buffer()),
            WebGlRenderingContext::STATIC_DRAW,
        );

        Buffer {
            attribute_name: String::from(name),
            attribute_location: None,
            value: Rc::new(gl_buffer),
            data_type: data_type,
            stride: 0,
            offset: 0,
            number_type: WebGlRenderingContext::UNSIGNED_BYTE,
        }
    }

    /// Function that looks up the attribute location associated with this `Buffer`.  
    /// To be used at initialization time, before any rendering occurs.
    pub fn lookup_location(
        &mut self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        if self.attribute_location == None {
            self.attribute_location =
                Some(context.get_attrib_location(program, self.attribute_name.as_str()));
        }
    }

    /// Enables and sets the attribute pointer at the context level.  
    /// Meant to be called just before rendering.
    pub fn enable_and_bind_attribute(&self, context: &WebGlRenderingContext) {
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.value));
        let location = self.attribute_location.unwrap() as u32;
        context.enable_vertex_attrib_array(location);
        context.vertex_attrib_pointer_with_i32(
            location,
            self.data_type.get_size(),
            self.number_type,
            false,
            self.stride,
            self.offset,
        );
    }
}
