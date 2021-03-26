//! Value types for the renderer for both Uniforms and Buffers.

use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use serde::{Deserialize, Serialize};
use std::slice;
use web_sys::{WebGlRenderingContext, WebGlUniformLocation};

use crate::asset::Texture;
use crate::error::Error;

#[derive(Serialize, Deserialize)]
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
    ) -> Result<(), Error> {
        match self {
            RendererValue::Float(f) => {
                context.uniform1fv_with_f32_array(location, slice::from_ref(f));
                Ok(())
            }
            RendererValue::FloatArray(f_array) => {
                context.uniform1fv_with_f32_array(location, f_array.as_slice());
                Ok(())
            }
            RendererValue::Vector2(vec) => {
                context.uniform2fv_with_f32_array(location, vec.as_slice());
                Ok(())
            }
            RendererValue::Vector2Array(arr) => {
                context.uniform2fv_with_f32_array(location, arr.as_slice());
                Ok(())
            }
            RendererValue::Vector3(vec) => {
                context.uniform3fv_with_f32_array(location, vec.as_slice());
                Ok(())
            }
            RendererValue::Vector3Array(arr) => {
                context.uniform3fv_with_f32_array(location, arr.as_slice());
                Ok(())
            }
            RendererValue::Vector4(vec) => {
                context.uniform4fv_with_f32_array(location, vec.as_slice());
                Ok(())
            }
            RendererValue::Vector4Array(arr) => {
                context.uniform4fv_with_f32_array(location, arr.as_slice());
                Ok(())
            }
            RendererValue::Matrix2(mat) => {
                context.uniform_matrix2fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            }
            RendererValue::Matrix3(mat) => {
                context.uniform_matrix3fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            }
            RendererValue::Matrix4(mat) => {
                context.uniform_matrix4fv_with_f32_array(location, false, mat.as_slice());
                Ok(())
            }
            RendererValue::Texture(tex) => match (&tex.value, texture_number) {
                (Some(val), Some(number)) => {
                    context.active_texture(get_texture_pointer(number));
                    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&val));
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
                (_, None) => Err(Error::UnknownTextureNumber),
                (None, _) => Err(Error::UnconstructedValue),
            }
        }
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
