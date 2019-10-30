//! Asset registry module

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::component::mesh::MeshData;
use crate::renderer::{material::{Material, MaterialInstance}};
use web_sys::WebGlRenderingContext;

/// Registry holding the `MeshData`, `Material`s, `MaterialInstance`s and Textures
/// to be used by the renderer at render time.
pub struct AssetRegistry {
    /// MeshData Registry
    mesh_data_registry: HashMap<String, Rc<MeshData>>,

    /// Material Registry
    material_registry: HashMap<String, Rc<RefCell<Material>>>,

    /// Material Instance Registry
    material_instance_registry: HashMap<String, Rc<RefCell<MaterialInstance>>>,
}

impl AssetRegistry {
    /// Constructor. Creates a new empty asset registry.
    pub fn new() -> AssetRegistry {
        AssetRegistry {
            mesh_data_registry: HashMap::new(),
            material_registry: HashMap::new(),
            material_instance_registry: HashMap::new(),
        }
    }

    /// Register mesh data from the byte array from a `MeshFile`
    pub fn register_mesh_data(
        &mut self,
        context: &WebGlRenderingContext,
        wmesh_data: &[u8],
    ) -> Result<String, String> {
        let mesh_data_result = super::deserialize_wmesh(context, wmesh_data);
        if let Ok(mesh_data) = mesh_data_result {
            let id = mesh_data.get_id().to_owned();
            self.mesh_data_registry
                .insert(id.clone(), Rc::new(mesh_data));
            Ok(id)
        } else {
            Err(String::from("Could not parse the mesh file!"))
        }
    }

    pub fn register_material_data(
        &mut self,
        context: &WebGlRenderingContext,
        wmaterial_data: &[u8],
    ) -> Result<String, String> {
        let mat_data_result = super::deserialize_wmaterial(context, wmaterial_data);
        if let Ok(material) = mat_data_result {
            let id = material.get_id().to_owned();
            self.material_registry
                .insert(id.clone(), Rc::new(RefCell::new(material)));
            Ok(id)
        } else {
            Err(String::from("Could not parse the mesh file!"))
        }
    }

    pub fn get_mesh_data(&self, id : &str) -> Option<Rc<MeshData>> {
        match self.mesh_data_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_material(&self, id : &str) -> Option<Rc<RefCell<Material>>> {
        match self.material_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_material_instance(&self, id : &str) -> Option<Rc<RefCell<MaterialInstance>>> {
        match self.material_instance_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }
}
