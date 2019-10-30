//! Interface and implementations for managing WebGl uniforms.
//!
//! Each Uniform is represented by a name and a value.
//!
//! Values can be of types
//!     - `f32`
//!     - `&[f32]`
//!     - `Vector2<f32>`
//!     - `&[Vector2<f32>]`
//!     - `Vector3<f32>`
//!     - `&[Vector3<f32>]`
//!     - `Vector4<f32>`
//!     - `&[Vector4<f32>]`
//!     - `Matrix2<f32>`
//!     - `Matrix3<f32>`
//!     - `Matrix4<f32>`

use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use std::slice;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use wtvr3d_file::ShaderDataType;

/// Name for the view-projection matrix uniform
pub const VP_MATRIX_NAME: &str = "u_vp_matrix";

/// Name for the view-projection matrix uniform
pub const WORLD_MATRIX_NAME: &str = "u_world_transform";

/// Name for the point lights matrix uniform
#[cfg(feature = "point_light")]
pub const POINT_LIGHTS_NAME: &str = "point_lights";

/// Name for the directional lights matrix uniform
#[cfg(feature = "directional_light")]
pub const DIRECTIONAL_LIGHTS_NAME: &str = "directional_lights";

/// Uniform representation; has a name and a value.  
/// Its location must be looked up at initialization time.
pub struct Uniform {
    /// Name of the uniform as it appears in the vertex or fragment shader
    pub name: String,

    /// Location of the uniform relative to a specific WebGlProgram
    location: Option<WebGlUniformLocation>,

    /// Value of the Uniform to pass to the program at render time.
    pub value: Box<dyn UniformValue>,
}

impl Uniform {
    /// Creates a new uniform from a name and value.
    pub fn new(name: &str, value: Box<dyn UniformValue>) -> Uniform {
        Uniform {
            name: name.to_owned(),
            location: None,
            value: value,
        }
    }

    /// Creates a uniform from a name, a value, and a pre-computed location.
    pub fn new_with_location(
        name: &str,
        location: Option<WebGlUniformLocation>,
        value: Box<dyn UniformValue>,
    ) -> Uniform {
        Uniform {
            name: name.to_owned(),
            location: location,
            value: value,
        }
    }

    /// Given a WebGlProgram, looks up the uniform location and saves it internally for future use.  
    /// Should be used at initialization time.
    pub fn lookup_location(
        &mut self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        if self.location == None {
            self.location = context.get_uniform_location(program, self.name.as_str())
        }
    }

    /// Sets the uniform to the current WebGlContext (to be called at render time);  
    /// The appropriate WebGlProgram must have been set beforehand.
    pub fn set_to_context(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        let result = self.value.set_to_context_at_location(
            context,
            if let Some(loc) = &self.location {
                Some(&loc)
            } else {
                None
            },
        );
        if let Err(_) = result {
            Err("Uniform couldn't be set".to_string())
        } else {
            result
        }
    }
}

/// Trait representing every type that can be a uniform value.
pub trait UniformValue {
    /// Given a location, sets the Uniform to the current context at render time.  
    /// The appropriate program must have been set.
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String>;
}

impl UniformValue for f32 {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        context.uniform1fv_with_f32_array(location, slice::from_ref(self));
        Ok(())
    }
}

impl UniformValue for &[f32] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_to_context_at_location(context, location)
    }
}

impl UniformValue for (ShaderDataType, &[f32]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        match self.0 {
            ShaderDataType::Single => {
                context.uniform1fv_with_f32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector2 => {
                context.uniform2fv_with_f32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector3 => {
                context.uniform3fv_with_f32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector4 => {
                context.uniform4fv_with_f32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Matrix2 => {
                context.uniform_matrix2fv_with_f32_array(location, false, self.1);
                Ok(())
            }
            ShaderDataType::Matrix3 => {
                context.uniform_matrix3fv_with_f32_array(location, false, self.1);
                Ok(())
            }
            ShaderDataType::Matrix4 => {
                context.uniform_matrix4fv_with_f32_array(location, false, self.1);
                Ok(())
            }
            _ => Err(String::from("Invalid value supplied to uniform")),
        }
    }
}

impl UniformValue for (ShaderDataType, Vec<f32>) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for i32 {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        context.uniform1iv_with_i32_array(location, slice::from_ref(self));
        Ok(())
    }
}

impl UniformValue for &[i32] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_to_context_at_location(context, location)
    }
}

impl UniformValue for (ShaderDataType, &[i32]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        match self.0 {
            ShaderDataType::Single => {
                context.uniform1iv_with_i32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector2 => {
                context.uniform2iv_with_i32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector3 => {
                context.uniform3iv_with_i32_array(location, self.1);
                Ok(())
            }
            ShaderDataType::Vector4 => {
                context.uniform4iv_with_i32_array(location, self.1);
                Ok(())
            }
            _ => Err(String::from("Invalid value supplied to uniform")),
        }
    }
}

impl UniformValue for (ShaderDataType, &[i16]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut new_vec = Vec::new();
        for i in self.1 {
            new_vec.push(*i as i32);
        }
        (self.0, new_vec.as_slice()).set_to_context_at_location(context, location)
    }
}
impl UniformValue for (ShaderDataType, Vec<i16>) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for (ShaderDataType, &[u8]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut new_vec = Vec::new();
        for i in self.1 {
            new_vec.push(*i as i32);
        }
        (self.0, new_vec.as_slice()).set_to_context_at_location(context, location)
    }
}
impl UniformValue for (ShaderDataType, Vec<u8>) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for Vector2<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector2, self.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for &[Vector2<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector2, vec.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for Vector3<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector3, self.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for &[Vector3<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector3, vec.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for Vector4<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector4, self.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for &[Vector4<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector4, vec.as_slice()).set_to_context_at_location(context, location)
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix2, self.as_slice()).set_to_context_at_location(context, location)
    }
}
impl UniformValue for Matrix3<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix3, self.as_slice()).set_to_context_at_location(context, location)
    }
}
impl UniformValue for Matrix4<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix4, self.as_slice()).set_to_context_at_location(context, location)
    }
}

pub struct GlobalUniformLocations {
    pub vp_matrix_location: Option<WebGlUniformLocation>,

    pub world_transform_location: Option<WebGlUniformLocation>,

    #[cfg(feature = "point_light")]
    pub point_lights_location: Option<WebGlUniformLocation>,

    #[cfg(feature = "directional_light")]
    pub directional_lights_location: Option<WebGlUniformLocation>,
}

impl GlobalUniformLocations {
    pub fn new() -> GlobalUniformLocations {
        GlobalUniformLocations {
            vp_matrix_location: None,
            world_transform_location: None,

            #[cfg(feature = "point_light")]
            point_lights_location: None,

            #[cfg(feature = "directional_light")]
            directional_lights_location: None,
        }
    }
    pub fn lookup_locations(
        &mut self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        if self.vp_matrix_location == None {
            self.vp_matrix_location = context.get_uniform_location(program, VP_MATRIX_NAME)
        }
        if self.world_transform_location == None {
            self.world_transform_location = context.get_uniform_location(program, VP_MATRIX_NAME)
        }

        #[cfg(feature = "point_light")]
        {
            if self.point_lights_location == None {
                self.point_lights_location =
                    context.get_uniform_location(program, POINT_LIGHTS_NAME)
            }
        }

        #[cfg(feature = "directional_light")]
        {
            if self.directional_lights_location == None {
                self.directional_lights_location =
                    context.get_uniform_location(program, DIRECTIONAL_LIGHTS_NAME)
            }
        }
    }
}
