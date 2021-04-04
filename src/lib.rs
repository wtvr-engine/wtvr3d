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

#[cfg(feature = "editor")]
mod editor;

#[cfg(feature = "debug")]
use console_error_panic_hook;
#[cfg(feature = "editor")]
pub use editor::Editor;
use wasm_bindgen::prelude::*;

/// Initialize the engine.
/// For now, this is only useful for debug.
/// This will be moved into Editor and Client implementations.
#[wasm_bindgen]
pub fn initialize() {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}
