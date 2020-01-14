//! Representation of a mesh in a scene

use crate::renderer::{LightConfiguration, Renderer};
use specs::{Component, VecStorage};
use std::cell::RefCell;
use std::rc::Rc;

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh {
    material: usize,
    material_instance: usize,
    mesh_data: usize,
}

impl Mesh {
    /// Constructor. Uses a `MeshData` id and a `MaterialInstance` id.
    pub fn new(mesh_data_id: usize, material_instance_id: usize, material_id: usize) -> Mesh {
        Mesh {
            mesh_data: mesh_data_id,
            material: material_id,
            material_instance: material_instance_id,
        }
    }
    /// Getter for material
    pub fn get_material_instance_id(&self) -> &usize {
        &self.material_instance
    }

    /// Getter for material
    pub fn get_material_id(&self) -> &usize {
        &self.material
    }

    /// Getter for mesh_data
    pub fn get_mesh_data_id(&self) -> &usize {
        &self.mesh_data
    }

    /// Compiles the material and fetches all the necessary uniform and attribute locations
    pub fn compile_material(
        &self,
        renderer_ref: Rc<RefCell<Renderer>>,
        light_config: &LightConfiguration,
    ) -> Result<(), String> {
        let renderer = renderer_ref.borrow();
        if let Some(material_rc) = renderer
            .get_asset_registry()
            .get_material_with_index(self.material)
        {
            {
                let mut material = material_rc.borrow_mut();
                if material.should_compile(light_config) {
                    match material.compile(renderer.get_webgl_context(), light_config) {
                        Err(message) => {
                            return Err(message);
                        }
                        _ => {}
                    }
                }
                material.lookup_locations(renderer.get_webgl_context(), light_config);
                material.light_configuration = light_config.clone();
            }
            if let Some(mesh) = renderer
                .get_asset_registry()
                .get_mesh_data_with_index(self.mesh_data)
            {
                mesh.borrow_mut()
                    .lookup_locations(renderer.get_webgl_context(), material_rc.clone());
            }
        } else {
            return Err("Material could not be found. Has it been registered yet?".to_owned());
        }
        if let Some(material_instance_rc) = renderer
            .get_asset_registry()
            .get_material_instance_with_index(self.material_instance)
        {
            let mut material_instance = material_instance_rc.borrow_mut();
            material_instance.lookup_locations(renderer.get_webgl_context(), light_config);
        } else {
            return Err(
                "Material Instance could not be found. Has it been registered yet?".to_owned(),
            );
        }
        Ok(())
    }
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}
