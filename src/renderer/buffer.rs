//! # Buffer
//!
//! Interface and implementations for managing Buffers

use super::ShaderDataType;
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

#[derive(Clone)]
pub struct Buffer {
    attribute_name: String,
    attribute_location: Option<i32>,
    value: Rc<WebGlBuffer>,
    data_type: ShaderDataType,
}

impl Buffer {
    pub fn from_f32_data(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: &[f32],
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(data);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        Buffer {
            attribute_name: String::from(name),
            attribute_location: None,
            value: Rc::new(gl_buffer),
            data_type: data_type,
        }
    }

    pub fn from_i32_data(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: &[i32],
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));
        unsafe {
            let vert_array = js_sys::Int32Array::view(data);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        Buffer {
            attribute_name: String::from(name),
            attribute_location: None,
            value: Rc::new(gl_buffer),
            data_type: data_type,
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

    pub fn enableAttribute(&self, context: &WebGlRenderingContext) {
        //to do
    }
}
