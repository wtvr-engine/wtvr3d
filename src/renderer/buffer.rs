//! # Buffer
//!
//! Interface and implementations for managing Buffers

use super::shader_data_type::ShaderDataType;
use js_sys::{Float32Array, Int16Array, Uint8Array};
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

#[derive(Clone)]
pub struct Buffer {
    attribute_name: String,
    attribute_location: Option<i32>,
    value: Rc<WebGlBuffer>,
    data_type: ShaderDataType,
    number_type: u32,
    pub stride: i32,
    pub offset: i32,
}

impl Buffer {
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
