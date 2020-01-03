//! Representation of a mesh in a scene

use specs::{Component, VecStorage};
use crate::renderer::{Renderer,LightConfiguration};
use std::rc::Rc;
use std::cell::RefCell;

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh {
    material: String,
    material_instance: String,
    mesh_data: String,
}

impl Mesh {
    /// Constructor. Uses a `MeshData` id and a `MaterialInstance` id.
    pub fn new(mesh_data_id: &str, material_instance_id: &str, material_id: &str) -> Mesh {
        Mesh {
            mesh_data: mesh_data_id.to_owned(),
            material: material_id.to_owned(),
            material_instance: material_instance_id.to_owned(),
        }
    }
    /// Getter for material
    pub fn get_material_instance_id(&self) -> &str {
        &self.material_instance
    }

    /// Getter for material
    pub fn get_material_id(&self) -> &str {
        &self.material
    }

    /// Getter for mesh_data
    pub fn get_mesh_data_id(&self) -> &str {
        &self.mesh_data
    }

    /// Compiles the material and fetches all the necessary uniform and attribute locations
    pub fn compile_material(&self, renderer_ref : Rc<RefCell<Renderer>>, light_config : &LightConfiguration) -> Result<(),String>{
        let renderer = renderer_ref.borrow();
        if let Some(material_rc) = renderer.get_asset_registry().get_material(&self.material) {
            let mut material = material_rc.borrow_mut();
            if material.should_compile(light_config) {
                match material.compile(renderer.get_webgl_context(),light_config){
                    Err(message) => { return Err(message);},
                    _ => {},
                }
            }
            material.lookup_locations(renderer.get_webgl_context(),light_config);
            if let Some(mesh) = renderer.get_asset_registry().get_mesh_data(&self.mesh_data) {
                mesh.lookup_locations(renderer.get_webgl_context(), material_rc.clone());
            }
            material.light_configuration = light_config.clone();
        }
        else {
            return Err("Material could not be found. Has it been registered yet?".to_owned());
        }
        if let Some(material_instance_rc) = renderer.get_asset_registry().get_material_instance(&self.material_instance) {
            let mut material_instance = material_instance_rc.borrow_mut();
            material_instance.lookup_locations(renderer.get_webgl_context(),light_config);
        }
        else {
            return Err("Material Instance could not be found. Has it been registered yet?".to_owned());
        }
        Ok(())
    }
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}
