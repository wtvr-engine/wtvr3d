//! This module defines data types and behaviours for assets : Materials, Meshes, etc.

mod material;

mod constructible;

mod file;

pub use material::Material;

pub use constructible::Constructible;

pub use file::File;
