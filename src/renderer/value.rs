//! Value types for the renderer for both Uniforms and Buffers.

use nalgebra::base::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use web_sys::{WebGlRenderingContext, WebGlUniformLocation};
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
#[non_exhaustive]
pub enum RendererValue {
    Float(f32),
    FloatArray(Vec<f32>),
    Vector2(Vector2<f32>),
    Vector2Array(Vec<Vector2<f32>>),
    Vector3(Vector3<f32>),
    Vector3Array(Vec<Vector3<f32>>),
    Vector4(Vector4<f32>),
    Vector4Array(Vec<Vector4<f32>>),
    Matrix2(Box<Matrix2<f32>>),
    Matrix3(Box<Matrix3<f32>>),
    Matrix4(Box<Matrix4<f32>>),
    Texture(usize),
}

impl RendererValue {
    pub fn set_to_context_as_uniform(
        &self,
        context: &WebGlRenderingContext,
        location: Option<&WebGlUniformLocation>,
        texture_number: Option<u32>,
    ) -> Result<(), String>{

        Ok(())
    }
}