//! Asset registry module

use crate::renderer::MeshData;
use crate::renderer::{Material, MaterialInstance};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

/// Registry holding the `MeshData`, `Material`s, `MaterialInstance`s and Textures
/// to be used by the renderer at render time.
pub struct AssetRegistry {
    /// MeshData Registry
    mesh_data_registry: HashMap<String, Rc<MeshData>>,

    /// Material Registry
    material_registry: HashMap<String, Rc<RefCell<Material>>>,

    /// Material Instance Registry
    material_instance_registry: HashMap<String, Rc<RefCell<MaterialInstance>>>,

    /// Material Instance Registry
    texture_registry: HashMap<String, Rc<RefCell<WebGlTexture>>>,
}

impl AssetRegistry {
    /// Constructor. Creates a new empty asset registry.
    pub fn new() -> AssetRegistry {
        AssetRegistry {
            mesh_data_registry: HashMap::new(),
            material_registry: HashMap::new(),
            material_instance_registry: HashMap::new(),
            texture_registry: HashMap::new(),
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

    /// Register a material from the byte array of a `MaterialFile`
    pub fn register_material(&mut self, wmaterial_data: &[u8]) -> Result<String, String> {
        let mat_data_result = super::deserialize_wmaterial(&self, wmaterial_data);
        match mat_data_result {
            Ok(material) => {
                let id = material.get_id().to_owned();
                self.material_registry
                    .insert(id.clone(), Rc::new(RefCell::new(material)));
                Ok(id)
            }
            Err(message) => Err(message),
        }
    }

    /// Register a material isntance from the byte array of a `MaterialInstanceFile`
    pub fn register_material_instance(&mut self, wmaterial_data: &[u8]) -> Result<String, String> {
        let mat_data_result = super::deserialize_wmatinstance(&self, wmaterial_data);
        match mat_data_result {
            Ok(matinstance) => {
                let id = matinstance.get_id().to_owned();
                self.material_instance_registry
                    .insert(id.clone(), Rc::new(RefCell::new(matinstance)));
                Ok(id)
            }
            Err(message) => Err(message),
        }
    }

    /// Register a new texture from an Image reference
    pub fn register_texture(
        &mut self,
        context: &WebGlRenderingContext,
        image: &HtmlImageElement,
        id: String,
    ) -> Result<String, String> {
        match context.create_texture() {
            None => Err(String::from("Could not create texture")),
            Some(texture) => {
                context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
                let res = context.tex_image_2d_with_u32_and_u32_and_image(
                    WebGlRenderingContext::TEXTURE_2D,
                    0,
                    WebGlRenderingContext::RGBA as i32,
                    WebGlRenderingContext::RGBA,
                    WebGlRenderingContext::UNSIGNED_BYTE,
                    image,
                );
                match res {
                    Err(_) => Err(String::from("Texture binding failed.")),
                    Ok(_) => {
                        self.texture_registry
                            .insert(id.clone(), Rc::new(RefCell::new(texture)));
                        Ok(id)
                    }
                }
            }
        }
    }

    pub fn get_mesh_data(&self, id: &str) -> Option<Rc<MeshData>> {
        match self.mesh_data_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_material(&self, id: &str) -> Option<Rc<RefCell<Material>>> {
        match self.material_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_material_instance(&self, id: &str) -> Option<Rc<RefCell<MaterialInstance>>> {
        match self.material_instance_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_texture(&self, id: &str) -> Option<Rc<RefCell<WebGlTexture>>> {
        match self.texture_registry.get(id) {
            Some(rc) => Some(rc.clone()),
            None => None,
        }
    }

    pub fn get_parent_material(&self, material_instance_id: &str) -> Option<Rc<RefCell<Material>>> {
        if let Some(material_instance) = self.get_material_instance(material_instance_id) {
            Some(material_instance.borrow().get_parent().clone())
        } else {
            None
        }
    }
}
