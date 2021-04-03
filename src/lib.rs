//! # wtvr3D: a lightweight and modular 3d library written in Rust with WebAssembly in mind
//!
//! **wtvr3d is still in very early stages of development, meaning it's not ready to be used yet.**
//!
//! wtvr3d's purpose is to  offer a WebAssembly alternative to popular JS 3d engines on the web, while ensuring performance and a small overall footprint.
//!
//! This version uses WebGL2 as its backend, as it should be widely available by the time this module is production-ready.

mod renderer;

mod asset;

mod error;

mod util;

mod importers;

use asset::{Constructible, Material};

#[cfg(feature = "debug")]
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen]
pub fn create_material(
    vertex_shader: &str,
    fragment_shader: &str,
    context: &WebGl2RenderingContext,
) -> String {
    let mut material = Material::new(
        "Default".to_string(),
        vertex_shader.to_string(),
        fragment_shader.to_string(),
        false,
        false,
    );
    material.construct(context, true).unwrap();
    String::from("Hello")
}

#[wasm_bindgen]
pub fn initialize() {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}
