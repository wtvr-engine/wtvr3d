//! Module for everything related to asset management.

mod material;

mod mesh;

mod asset_registry;

pub use material::{Material,UniformOverrides};
pub use mesh::Mesh;
pub use asset_registry::AssetRegistry;

use web_sys::WebGlTexture;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
#[non_exhaustive]
pub enum Asset {
    Mesh(Box<Mesh>),
    Material(Box<Material>),
    UniformOverrides(Box<UniformOverrides>),
    #[serde(skip)]
    Texture(Box<WebGlTexture>),
}