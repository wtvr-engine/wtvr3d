//! Interface and implementations for managing WebGl uniforms.
//!
//! Each Uniform is represented by a name and a value.
//!
//! Values can be of types
//!     - `f32`
//!     - `&[f32]`
//!     - `Vector2<f32>`
//!     - `Vector3<f32>`
//!     - `Vector4<f32>`
//!     - `Matrix2<f32>`
//!     - `Matrix3<f32>`
//!     - `Matrix4<f32>`

use crate::{error::Error, renderer::value::RendererValue};
use serde::{Deserialize, Serialize};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

/// Uniform representation; has a name and a value.  
/// Its location must be looked up at initialization time.
#[derive(Serialize, Deserialize)]
pub struct Uniform {
    /// Name of the uniform as it appears in the vertex or fragment shader
    pub name: String,

    /// Location of the uniform relative to a specific WebGlProgram
    #[serde(skip)]
    location: Option<WebGlUniformLocation>,

    /// Value of the Uniform to pass to the program at render time.
    pub value: RendererValue,

    /// Index of the texture buffer to which the texture has been bound in the `WebGlRenderingContext`
    texture_index: Option<u32>,
}

impl Uniform {
    /// Creates a new uniform from a name and value.
    pub fn new(name: &str, value: RendererValue) -> Uniform {
        Uniform {
            name: name.to_owned(),
            location: None,
            value,
            texture_index: None,
        }
    }

    /// Creates a uniform from a name, a value, and a pre-computed location.
    pub fn new_with_location(
        name: &str,
        location: Option<WebGlUniformLocation>,
        value: RendererValue,
    ) -> Uniform {
        Uniform {
            name: name.to_owned(),
            location,
            value,
            texture_index: None,
        }
    }

    pub fn set_texture_index(&mut self, index: u32) {
        self.texture_index = Some(index);
    }

    pub fn get_texture_index(&self) -> Option<u32> {
        self.texture_index
    }

    /// Given a WebGlProgram, looks up the uniform location and saves it internally for future use.  
    /// Should be used at initialization time.
    pub fn lookup_location(
        &mut self,
        context: &WebGlRenderingContext,
        program: &Option<WebGlProgram>,
    ) -> () {
        if self.location == None {
            self.location =
                context.get_uniform_location(program.as_ref().unwrap(), self.name.as_str())
        }
    }

    /// Sets the uniform to the current WebGlContext (to be called at render time);  
    /// The appropriate WebGlProgram must have been set beforehand.
    pub fn set_to_context(&self, context: &WebGlRenderingContext) -> Result<(), Error> {
        let result = self.value.set_to_context_as_uniform(
            context,
            if let Some(loc) = &self.location {
                Some(&loc)
            } else {
                None
            },
            self.texture_index,
        );
        if let Err(_) = result {
            Err(Error::UniformError)
        } else {
            result
        }
    }
}
