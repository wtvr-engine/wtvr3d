//! Interface and implementations for managing WebGL Buffers and Attributes.

use js_sys::{Float32Array, Uint16Array};
use std::rc::Rc;
use web_sys::{WebGlBuffer, WebGlRenderingContext};
use wtvr3d_file::ShaderDataType;



/// ## Buffer
///
/// A `Buffer` reprensents information about an attribute and its buffer.  
/// It contains its attribute's name, location, buffer value, as well as
/// its data type and buffer offset.
#[derive(Clone)]
pub struct Buffer {
    /// Attribute name for the current Buffer.
    attribute_name: String,

    /// Buffer reference; several `Buffer` objects can use the same `WebGlBuffer`
    value: Rc<WebGlBuffer>,

    /// Index Buffer reference; several `Buffer` objects can use the same index `WebGlBuffer`
    indexes : Option<Rc<WebGlBuffer>>,

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
    
    pub fn from_f32_data_view(
        context: &WebGlRenderingContext,
        name: &str,
        data_type: ShaderDataType,
        data: &[f32],
        indexes : Option<&[u16]>,
    ) -> Buffer {
        let gl_buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&gl_buffer));
        
        unsafe {
            let float_array = Float32Array::view(data);
            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &float_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        
        let mut indexes_buffer = None;
        if let Some(indexes_array) = indexes {
            if indexes_array.len() > 0 {
                let gl_index_buffer = context.create_buffer().unwrap();
                context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&gl_index_buffer));
                
                unsafe {
                    let uint_array = Uint16Array::view(indexes_array);
                    context.buffer_data_with_array_buffer_view(
                        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                        &uint_array,
                        WebGlRenderingContext::STATIC_DRAW,
                    );
                }
                indexes_buffer = Some(Rc::new(gl_index_buffer));
            }
            
        }
        
        
        Buffer {
            attribute_name: String::from(name),
            value: Rc::new(gl_buffer),
            indexes : indexes_buffer,
            data_type: data_type,
            stride: 0,
            offset: 0,
            number_type: WebGlRenderingContext::FLOAT,
        }
    }
    
    /// Returns the attribute name for this buffer
    pub fn get_attribute_name(&self) -> &str {
        self.attribute_name.as_str()
    }
    
    /// Enables and sets the attribute pointer at the context level.  
    /// Meant to be called just before rendering.
    pub fn enable_and_bind_attribute(&self, context: &WebGlRenderingContext, location: i32) {
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.value));
        if let Some(index_buffer) = &self.indexes {
            context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,Some(&index_buffer));
        }
        let loc = location as u32;
        if location != -1 {
            context.enable_vertex_attrib_array(loc);
            context.vertex_attrib_pointer_with_i32(
                loc,
                self.data_type.get_size(),
                self.number_type,
                false,
                self.stride,
                self.offset,
            );
        }
    }
}
