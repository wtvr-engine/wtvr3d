//! Convenient interface for managing texture as actual WebGLTexture or texture ids.
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use web_sys::WebGlTexture;

#[derive(Serialize, Deserialize)]
pub struct Texture {
    pub id: usize,
    #[serde(skip)]
    pub value: Option<Rc<WebGlTexture>>,
}
