//! Module for everything exclusive to the editor version of the library.

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod asset_database;

use self::asset_database::AssetDatabase;

/// Editor is the struct that gives access to the editor-exclusive features
/// through wasm-bindgen.
/// It allows importing and exporting assets in WTVR3D format, and keeps track of them
#[wasm_bindgen]
pub struct Editor {
    asset_database : AssetDatabase,
}

#[wasm_bindgen]
impl Editor {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Editor {
        Editor {
            asset_database : AssetDatabase::new(),
        }
    }

    pub fn import_collada_mesh(&mut self, name: &str, dae_file : &str , context : &WebGl2RenderingContext) -> Result<(), JsValue> {
        self.asset_database.import_collada_mesh(name, dae_file, context)
    }

    pub fn create_material(&mut self, name: &str, vertex_shader : &str, fragment_shader : &str, lit :bool, transparent : bool, context : &WebGl2RenderingContext) -> Result<usize, JsValue> {
        self.asset_database.create_material(name, vertex_shader, fragment_shader, lit, transparent, context)
    }

}