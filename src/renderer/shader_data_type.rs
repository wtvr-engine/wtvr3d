//! GLGSL data type enumeration and associated component count.

/// Shader data type enum.  
/// Enumerates every shader data type that can be used with `wtvr3d`
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
    /// Gets the size in number of components for a any data type.  
    /// Usage: give component size when binding attributes to buffers.
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
