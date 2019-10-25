//! # shader_data_type
//!
//! GLGSL data types and relevant information

#[derive(Clone)]
pub enum ShaderDataType {
    Single,
    Vector2,
    Vector3,
    Vector4,
    Matrix2,
    Matrix3,
    Matrix4,
    Sampler2D,
}

impl ShaderDataType {
    pub fn get_size(&self) -> i32 {
        match &self {
            ShaderDataType::Vector2 => 2,
            ShaderDataType::Vector3 => 3,
            ShaderDataType::Vector4 | ShaderDataType::Matrix2 => 4,
            ShaderDataType::Matrix3 => 9,
            ShaderDataType::Matrix4 => 16,
            ShaderDataType::Single | _ => 1,
        }
    }
}
