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

use crate::renderer::LightConfiguration;
use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use std::cell::RefCell;
use std::rc::Rc;
use std::slice;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlTexture, WebGlUniformLocation};
use wtvr3d_file::ShaderDataType;

/// Name for the view matrix uniform
pub const VIEW_MATRIX_NAME: &str = "u_view_matrix";

/// Name for the view matrix uniform
pub const PROJECTION_MATRIX_NAME: &str = "u_projection_matrix";

/// Name for the world transform (model) matrix uniform
pub const WORLD_TRANSFORM_NAME: &str = "u_world_transform";

/// Name for the ambiant light uniform
pub const AMBIANT_LIGHT_NAME: &str = "u_ambiant_light";

/// Name for the point lights array uniform
pub const POINT_LIGHTS_NAME: &str = "u_point_lights";

/// Name for the directional lights array uniform
pub const DIRECTIONAL_LIGHTS_NAME: &str = "u_dir_lights";

/// Name for the color field in the Light GLSL struct
pub const LIGHT_COLOR_NAME: &str = "color";

/// Name for the intensity field in the Light GLSL struct
pub const LIGHT_INTENSITY_NAME: &str = "intensity";

/// Name for the attenuation field in the Light GLSL struct
pub const LIGHT_ATTENUATION_NAME: &str = "attenuation";

/// Name for the direction/position field in the Light GLSL struct
pub const LIGHT_POSITION_DIRECTION_NAME: &str = "position_or_direction";

/// Maximum number of lights of each type
pub const MAX_NUMBER_OF_LIGHTS: usize = 8;

/// Uniform representation; has a name and a value.  
/// Its location must be looked up at initialization time.
pub struct Uniform {
    /// Name of the uniform as it appears in the vertex or fragment shader
    pub name: String,

    /// Location of the uniform relative to a specific WebGlProgram
    location: Option<WebGlUniformLocation>,

    /// Value of the Uniform to pass to the program at render time.
    pub value: Box<dyn UniformValue>,

    /// Index of the texture buffer to which the texture has been bound in the `WebGlRenderingContext`
    texture_index: Option<u32>,
}

impl Uniform {
    /// Creates a new uniform from a name and value.
    pub fn new(name: &str, value: Box<dyn UniformValue>) -> Uniform {
        Uniform {
            name: name.to_owned(),
            location: None,
            value: value,
            texture_index: None,
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
    pub fn set_to_context(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        let result = self.value.set_to_context_at_location(
            context,
            if let Some(loc) = &self.location {
                Some(&loc)
            } else {
                None
            },
            self.texture_index,
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
        texture_number: Option<u32>,
    ) -> Result<(), String>;
}

impl UniformValue for f32 {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
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
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for Rc<RefCell<WebGlTexture>> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        match texture_number {
            None => Err(String::from(
                "You must provide a texture number for Texture uniforms",
            )),
            Some(number) => {
                context.active_texture(get_texture_pointer(number));
                context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.borrow()));
                context.tex_parameteri(
                    WebGlRenderingContext::TEXTURE_2D,
                    WebGlRenderingContext::TEXTURE_MAG_FILTER,
                    WebGlRenderingContext::LINEAR as i32,
                );
                context.tex_parameteri(
                    WebGlRenderingContext::TEXTURE_2D,
                    WebGlRenderingContext::TEXTURE_MIN_FILTER,
                    WebGlRenderingContext::NEAREST as i32,
                );
                context.uniform1i(location, number as i32);
                Ok(())
            }
        }
    }
}

impl UniformValue for (ShaderDataType, &[f32]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
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
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location, texture_number)
    }
}

impl UniformValue for i32 {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
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
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Single, *self).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for (ShaderDataType, &[i32]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
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
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        let mut new_vec = Vec::new();
        for i in self.1 {
            new_vec.push(*i as i32);
        }
        (self.0, new_vec.as_slice()).set_to_context_at_location(context, location, texture_number)
    }
}
impl UniformValue for (ShaderDataType, Vec<i16>) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location, texture_number)
    }
}

impl UniformValue for (ShaderDataType, &[u8]) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        let mut new_vec = Vec::new();
        for i in self.1 {
            new_vec.push(*i as i32);
        }
        (self.0, new_vec.as_slice()).set_to_context_at_location(context, location, texture_number)
    }
}
impl UniformValue for (ShaderDataType, Vec<u8>) {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (self.0, self.1.as_slice()).set_to_context_at_location(context, location, texture_number)
    }
}

impl UniformValue for Vector2<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector2, self.as_slice()).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for &[Vector2<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector2, vec.as_slice()).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for Vector3<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector3, self.as_slice()).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for &[Vector3<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector3, vec.as_slice()).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for Vector4<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Vector4, self.as_slice()).set_to_context_at_location(
            context,
            location,
            texture_number,
        )
    }
}

impl UniformValue for &[Vector4<f32>] {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
    ) -> Result<(), String> {
        let mut vec: Vec<f32> = Vec::new();
        for vector in self.iter() {
            vec.splice(self.len()..self.len(), vector.as_slice().iter().cloned());
        }
        (ShaderDataType::Vector4, vec.as_slice())
            .set_to_context_at_location(context, location, None)
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix2, self.as_slice())
            .set_to_context_at_location(context, location, None)
    }
}
impl UniformValue for Matrix3<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix3, self.as_slice())
            .set_to_context_at_location(context, location, None)
    }
}
impl UniformValue for Matrix4<f32> {
    fn set_to_context_at_location(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        _texture_number: Option<u32>,
    ) -> Result<(), String> {
        (ShaderDataType::Matrix4, self.as_slice())
            .set_to_context_at_location(context, location, None)
    }
}

pub struct GlobalUniformLocations {
    pub view_matrix_location: Option<WebGlUniformLocation>,

    pub projection_matrix_location: Option<WebGlUniformLocation>,

    pub world_transform_location: Option<WebGlUniformLocation>,

    pub ambiant_light_location: Option<WebGlUniformLocation>,

    pub point_lights_locations: [LightUniformLocations; MAX_NUMBER_OF_LIGHTS],

    pub point_lights_number_location: Option<WebGlUniformLocation>,

    pub directional_lights_locations: [LightUniformLocations; MAX_NUMBER_OF_LIGHTS],

    pub directional_lights_number_location: Option<WebGlUniformLocation>,
}

impl GlobalUniformLocations {
    pub fn new() -> GlobalUniformLocations {
        GlobalUniformLocations {
            view_matrix_location: None,
            projection_matrix_location: None,
            world_transform_location: None,

            ambiant_light_location: None,

            point_lights_locations: Default::default(),
            point_lights_number_location: None,

            directional_lights_locations: Default::default(),
            directional_lights_number_location: None,
        }
    }
    pub fn lookup_locations(
        &mut self,
        context: &WebGlRenderingContext,
        program: &Option<WebGlProgram>,
        light_config: &LightConfiguration,
    ) -> () {
        let pg = program.as_ref().unwrap();
        if self.view_matrix_location == None {
            self.view_matrix_location = context.get_uniform_location(pg, VIEW_MATRIX_NAME)
        }
        if self.projection_matrix_location == None {
            self.projection_matrix_location =
                context.get_uniform_location(pg, PROJECTION_MATRIX_NAME)
        }
        if self.world_transform_location == None {
            self.world_transform_location = context.get_uniform_location(pg, WORLD_TRANSFORM_NAME)
        }

        if self.ambiant_light_location == None {
            self.ambiant_light_location = context.get_uniform_location(pg, AMBIANT_LIGHT_NAME)
        }

        for i in 0..light_config.directional {
            &self.directional_lights_locations[i].lookup_locations(
                DIRECTIONAL_LIGHTS_NAME,
                Some(i),
                context,
                pg,
            );
        }

        for i in 0..light_config.point {
            &self.point_lights_locations[i].lookup_locations(
                POINT_LIGHTS_NAME,
                Some(i),
                context,
                pg,
            );
        }
    }
}

#[derive(Default)]
pub struct LightUniformLocations {
    pub color: Option<WebGlUniformLocation>,
    pub intensity: Option<WebGlUniformLocation>,
    pub attenuation: Option<WebGlUniformLocation>,
    pub position_or_direction: Option<WebGlUniformLocation>,
}

impl LightUniformLocations {
    pub fn new() -> LightUniformLocations {
        LightUniformLocations {
            color: None,
            intensity: None,
            attenuation: None,
            position_or_direction: None,
        }
    }

    pub fn lookup_locations(
        &mut self,
        light_type: &str,
        light_index: Option<usize>,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> () {
        if self.color == None {
            self.color = LightUniformLocations::lookup_field_location(
                light_type,
                LIGHT_COLOR_NAME,
                light_index,
                context,
                program,
            );
        }
        if self.intensity == None {
            self.intensity = LightUniformLocations::lookup_field_location(
                light_type,
                LIGHT_INTENSITY_NAME,
                light_index,
                context,
                program,
            );
        }
        if self.attenuation == None {
            self.attenuation = LightUniformLocations::lookup_field_location(
                light_type,
                LIGHT_ATTENUATION_NAME,
                light_index,
                context,
                program,
            );
        }
        if self.position_or_direction == None {
            self.position_or_direction = LightUniformLocations::lookup_field_location(
                light_type,
                LIGHT_POSITION_DIRECTION_NAME,
                light_index,
                context,
                program,
            );
        }
    }

    fn lookup_field_location(
        light_type: &str,
        field: &str,
        light_index: Option<usize>,
        context: &WebGlRenderingContext,
        program: &WebGlProgram,
    ) -> Option<WebGlUniformLocation> {
        let uniform_name = match light_index {
            Some(i) => format!("{}[{}].{}", light_type, i, field),
            None => format!("{}.{}", light_type, field),
        };
        context.get_uniform_location(program, &uniform_name)
    }
}

fn get_texture_pointer(texture_number: u32) -> u32 {
    match texture_number {
        0 => WebGlRenderingContext::TEXTURE0,
        1 => WebGlRenderingContext::TEXTURE1,
        2 => WebGlRenderingContext::TEXTURE2,
        3 => WebGlRenderingContext::TEXTURE3,
        4 => WebGlRenderingContext::TEXTURE4,
        5 => WebGlRenderingContext::TEXTURE5,
        6 => WebGlRenderingContext::TEXTURE6,
        7 => WebGlRenderingContext::TEXTURE7,
        _ => WebGlRenderingContext::TEXTURE8,
    }
}
