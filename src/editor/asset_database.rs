//! Asset Database module.

use crate::asset::{Constructible, Material, Mesh};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
pub struct AssetDatabase {
    loaded_meshes: Vec<Mesh>,
    loaded_materials: Vec<Material>,
}

impl AssetDatabase {
    pub fn new() -> AssetDatabase {
        AssetDatabase {
            loaded_meshes: Vec::new(),
            loaded_materials: Vec::new(),
        }
    }
    pub fn import_collada_mesh(
        &mut self,
        name: &str,
        dae_file: &str,
        context: &WebGl2RenderingContext,
    ) -> Result<(), JsValue> {
        let mut meshes = Mesh::from_collada(dae_file, name)?;
        for mesh in &mut meshes {
            mesh.construct(context)?;
        }
        self.loaded_meshes.append(&mut meshes);
        Ok(())
    }
    pub fn create_material(
        &mut self,
        name: &str,
        vertex_shader: &str,
        fragment_shader: &str,
        lit: bool,
        transparent: bool,
        context: &WebGl2RenderingContext,
    ) -> Result<usize, JsValue> {
        let mut material = Material::new(
            name.to_string(),
            vertex_shader.to_string(),
            fragment_shader.to_string(),
            lit,
            transparent,
        );
        material.construct(context)?;
        self.loaded_materials.push(material);
        Ok(self.loaded_materials.len() - 1)
    }
}
