//! Module for everything related to asset management.

mod material;

mod mesh;

mod texture;

mod asset_registry;

pub use asset_registry::AssetRegistry;
pub use material::{Material, UniformOverrides};
pub use mesh::Mesh;
pub use texture::Texture;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Asset {
    Mesh(Box<Mesh>),
    Material(Box<Material>),
    UniformOverrides(Box<UniformOverrides>),
    Texture(Texture),
}
