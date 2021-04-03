//! Module for everything constituting a Mesh

use js_sys::{Float32Array, Uint32Array};
use serde::{Deserialize, Serialize};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::error::W3DError;

use super::Constructible;

#[derive(Serialize, Deserialize)]
pub enum BufferData {
    F32(Vec<f32>),
    U32(Vec<u32>),
}
/// Buffer wrapper object; represents vertex data, index data, normals, etc.
#[derive(Serialize, Deserialize)]
pub struct Buffer {
    /// Name of the attribute for that buffer
    attribute_name: String,

    /// WebGl buffer for that Buffer object. Constructed value.
    #[serde(skip)]
    value: Option<WebGlBuffer>,

    /// Actual mesh data. May be cleaned once buffer is created.
    data: Option<BufferData>,
}

impl Buffer {
    #[cfg(feature = "import_collada")]
    pub fn new_from_f32_data(attribute_name: String, data: Vec<f32>) -> Buffer {
        Buffer {
            attribute_name,
            value: None,
            data: Some(BufferData::F32(data)),
        }
    }
    #[cfg(feature = "import_collada")]
    pub fn new_from_u32_data(attribute_name: String, data: Vec<u32>) -> Buffer {
        Buffer {
            attribute_name,
            value: None,
            data: Some(BufferData::U32(data)),
        }
    }
}

impl Constructible for Buffer {
    fn construct(
        &mut self,
        context: &WebGl2RenderingContext,
        clean_up: bool,
    ) -> Result<(), crate::error::W3DError> {
        let gl_buffer = context.create_buffer().ok_or_else(|| {
            W3DError::new(
                "Could not construct buffer",
                Some(self.attribute_name.clone()),
            )
        })?;
        match &self.data {
            Some(BufferData::F32(_)) => {
                context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&gl_buffer))
            }
            Some(BufferData::U32(_)) => context.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&gl_buffer),
            ),
            None => {
                return Err(W3DError::new(
                    "Trying to construct buffer without data",
                    Some(self.attribute_name.clone()),
                ));
            }
        }

        match &self.data {
            Some(BufferData::F32(data)) => unsafe {
                let view = Float32Array::view(data);
                context.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            },
            Some(BufferData::U32(data)) => unsafe {
                let view = Uint32Array::view(data);
                context.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                    &view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            },
            _ => {}
        };
        self.value = Some(gl_buffer);
        if clean_up {
            self.data = None;
        }
        Ok(())
    }

    fn is_constructed(&self) -> bool {
        self.value.is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mesh {}
