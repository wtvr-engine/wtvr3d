//! # Uniform
//!
//! Interface and implementations for managing uniforms

use super::shader_data_type::ShaderDataType;
use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use std::slice;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

pub const VP_MATRIX_NAME: &str = "vp_matrix";

#[cfg(feature = "point_light")]
pub const POINT_LIGHTS_NAME: &str = "point_lights";

#[cfg(feature = "directional_light")]
pub const DIRECTIONAL_LIGHTS_NAME: &str = "directional_lights";

pub struct Uniform<'a> {
    pub name: &'a str,
    location: Option<WebGlUniformLocation>,
    pub value: Box<dyn UniformValue>,
}

impl<'a> Uniform<'a> {
    pub fn new(name: &'a str, value: Box<dyn UniformValue>) -> Uniform {
        Uniform {
            name: name,
            location: None,
            value: value,
        }
    }

    pub fn new_with_location(
        name: &'a str,
        location: Option<WebGlUniformLocation>,
        value: Box<dyn UniformValue>,
    ) -> Uniform {
        Uniform {
            name: name,
            location: location,
            value: value,
        }
    }

    pub fn lookup_location(
        &mut self,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        if self.location == None {
            self.location = context.get_uniform_location(program, self.name)
        }
    }

    pub fn set(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        let result = self.value.set_uniform(
            context,
            if let Some(loc) = &self.location {
                Some(&loc)
            } else {
                None
            },
        );
        if let Err(message) = result {
            Err(format!("Uniform {} couldn't be set; {}", self.name, message).to_string())
        } else {
            result
        }
    }
}

pub trait UniformValue {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String>;
}

impl UniformValue for f32 {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        context.uniform1fv_with_f32_array(location, slice::from_ref(self));
        Ok(())
    }
}

impl UniformValue for &[f32] {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_uniform(context, location)
    }
}

impl UniformValue for (ShaderDataType, &[f32]) {
    fn set_uniform(
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

impl UniformValue for i32 {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        context.uniform1iv_with_i32_array(location, slice::from_ref(self));
        Ok(())
    }
}

impl UniformValue for &[i32] {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_uniform(context, location)
    }
}

impl UniformValue for (ShaderDataType, &[i32]) {
    fn set_uniform(
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

impl UniformValue for Vector2<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector2, self.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for &[Vector2<f32>] {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector2, vec.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for Vector3<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector3, self.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for &[Vector3<f32>] {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector3, vec.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for Vector4<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector4, self.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for &[Vector4<f32>] {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector4, vec.as_slice()).set_uniform(context, location)
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix2, self.as_slice()).set_uniform(context, location)
    }
}
impl UniformValue for Matrix3<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix3, self.as_slice()).set_uniform(context, location)
    }
}
impl UniformValue for Matrix4<f32> {
    fn set_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix4, self.as_slice()).set_uniform(context, location)
    }
}

pub struct GlobalUniformLocations {
    pub vp_matrix_location: Option<WebGlUniformLocation>,

    #[cfg(feature = "point_light")]
    pub point_lights_location: Option<WebGlUniformLocation>,

    #[cfg(feature = "directional_light")]
    pub directional_lights_location: Option<WebGlUniformLocation>,
}

impl GlobalUniformLocations {
    pub fn new() -> GlobalUniformLocations {
        GlobalUniformLocations {
            vp_matrix_location: None,

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
