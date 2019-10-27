//! # wtvr3D: a lightweight and modular 3d library written in Rust with WebAssembly in mind
//!
//! **wtvr3d is still in very early stages of development, meaning it's not ready to be used yet.**
//!
//! wtvr3d's purpose is to  offer a WebAssembly alternative to popular JS 3d engines on the web, while ensuring performance and a small overall footprint.

pub mod component;
pub mod renderer;
pub mod scene;
pub mod utils;

#[cfg(feature = "tests")]
pub mod tests;
