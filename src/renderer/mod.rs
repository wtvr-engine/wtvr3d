//! Rendering Engine for wtvr3d. Uses WebGL through the `web-sys` crate.

mod value;

mod uniform;

mod buffer;

pub use uniform::Uniform;

pub use buffer::Buffer;
