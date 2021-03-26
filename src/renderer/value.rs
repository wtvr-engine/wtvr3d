//! Value types for the renderer for both Uniforms and Buffers.

use std::slice;
use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use web_sys::{WebGlRenderingContext, WebGlUniformLocation};
use serde::{Deserialize,Serialize};

use crate::asset::Texture;
use crate::error::Error;

#[derive(Serialize,Deserialize)]
#[non_exhaustive]
pub enum RendererValue {
    Float(f32),
    FloatArray(Vec<f32>),
    Vector2(Vector2<f32>),
    Vector2Array(Vec<f32>),
    Vector3(Vector3<f32>),
    Vector3Array(Vec<f32>),
    Vector4(Vector4<f32>),
    Vector4Array(Vec<f32>),
    Matrix2(Box<Matrix2<f32>>),
    Matrix3(Box<Matrix3<f32>>),
    Matrix4(Box<Matrix4<f32>>),
    Texture(Texture),
}

impl RendererValue {
    pub fn set_to_context_as_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), Error>{
        match self {
            RendererValue::Float(f) => {
                context.uniform1fv_with_f32_array(location, slice::from_ref(f));
                Ok(())
            },
            RendererValue::FloatArray(f_array) => {
                context.uniform1fv_with_f32_array(location, f_array.as_slice());
                Ok(())
            },
            RendererValue::Vector2(vec) => {
                context.uniform2fv_with_f32_array(location, vec.as_slice());
                Ok(())
            },
            RendererValue::Vector2Array(arr) => {
                context.uniform2fv_with_f32_array(location, arr.as_slice());
                Ok(())
            },
            RendererValue::Vector3(vec) => {
                context.uniform3fv_with_f32_array(location, vec.as_slice());
                Ok(())
            },
            RendererValue::Vector3Array(arr) => {
                context.uniform3fv_with_f32_array(location, arr.as_slice());
                Ok(())
            },
            RendererValue::Vector4(vec) => {
                context.uniform4fv_with_f32_array(location, vec.as_slice());
                Ok(())
            },
            RendererValue::Vector3Array(arr) => {
                context.uniform4fv_with_f32_array(location, arr.as_slice());
                Ok(())
            },
            RendererValue::Matrix2(mat) => {
                context.uniform_matrix2fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            },
            RendererValue::Matrix3(mat) => {
                context.uniform_matrix3fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            },
            RendererValue::Matrix4(mat) => {
                context.uniform_matrix4fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            },

            _ => Err(Error::Unimplemented),
        }
    }
}