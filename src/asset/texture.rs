//! Convenient interface for managing texture as actual WebGLTexture or texture ids.
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::WebGlTexture;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Texture {
    pub id : usize,
    #[serde(skip)]
    pub texture : Option<Rc<RefCell<WebGlTexture>>>,
}