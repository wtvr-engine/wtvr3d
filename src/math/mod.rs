//! # math
//!
//! A module that provides most of the mathematical requirements to create a full 3d engine, like vectors and matrices along with their common operations.
pub use self::vector::Vector3;

pub use self::matrix::Matrix4;

pub use self::quaternion::Quaternion;


pub mod vector;

pub mod matrix;

pub mod quaternion;

pub mod color;

pub const PI : f32 =  3.14159265359 as f32;
