//! Convenient interface for managing texture as actual WebGLTexture or texture ids.
use serde::{Deserialize, Serialize};
use web_sys::WebGlTexture;

#[derive(Serialize, Deserialize)]
pub struct Texture {
    pub id: usize,
    #[serde(skip)]
    pub texture: Option<WebGlTexture>,
}
