//! Asset registry module

use crate::renderer::MeshData;
use crate::renderer::{Material, MaterialInstance};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

#[non_exhaustive]
pub enum Asset {
    MeshData(Rc<RefCell<MeshData>>),
    Material(Rc<RefCell<Material>>),
    MaterialInstance(Rc<RefCell<MaterialInstance>>),
    Texture(Rc<WebGlTexture>),
    None,
}

/// Registry holding the `MeshData`, `Material`s, `MaterialInstance`s and Textures
/// to be used by the renderer at render time.
pub struct AssetRegistry {

    /// Contains a collection of assets. 
    /// They can be queried using the index to find the position an asset with a specific
    /// String id occupies.
    assets : Vec<Asset>,

    /// Index linking each initial String ID to an internal usize ID.
    index : HashMap<String,usize>,
}

impl AssetRegistry {
    /// Constructor. Creates a new empty asset registry.
    pub fn new() -> AssetRegistry {
        AssetRegistry {
            assets : Vec::new(),
            index : HashMap::new(),
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
            self.index.insert(id.clone(), self.assets.len());
            self.assets.push(Asset::MeshData(Rc::new(RefCell::new(mesh_data))));
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
                self.index.insert(id.clone(), self.assets.len());
                self.assets.push(Asset::Material(Rc::new(RefCell::new(material))));
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
                self.index.insert(id.clone(), self.assets.len());
                self.assets.push(Asset::MaterialInstance(Rc::new(RefCell::new(matinstance))));
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
                        self.index.insert(id.clone(), self.assets.len());
                        self.assets.push(Asset::Texture(Rc::new(texture)));
                        Ok(id)
                    }
                }
            }
        }
    }

    pub fn get_id_from_str(&self, str_id : &str) -> Option<usize>{
        self.index.get(str_id).map(|id| id.to_owned())
    }

    fn get_asset(&self, id: &str) -> &Asset {
        match self.index.get(id) {
            Some(asset) => &self.assets[asset.to_owned()],
            None => &Asset::None,
        }
    }

    pub fn get_mesh_data(&self, id: &str) -> Option<Rc<RefCell<MeshData>>> {
        match self.get_asset(id) {
            Asset::MeshData(rc) => Some(rc.clone()),
            _ => None,
        }
    }

    pub fn get_material(&self, id: &str) -> Option<Rc<RefCell<Material>>> {
        match self.get_asset(id) {
            Asset::Material(rc) => Some(rc.clone()),
            _ => None,
        }
    }

    pub fn get_material_instance(&self, id: &str) -> Option<Rc<RefCell<MaterialInstance>>> {
        match self.get_asset(id) {
            Asset::MaterialInstance(rc) => Some(rc.clone()),
            _ => None,
        }
    }

    pub fn get_texture(&self, id: &str) -> Option<Rc<WebGlTexture>> {
        match self.get_asset(id) {
            Asset::Texture(rc) => Some(rc.clone()),
            _ => None,
        }
    }

    pub fn get_mesh_data_with_index(&self, id: usize) -> Option<Rc<RefCell<MeshData>>> {
        if id < self.assets.len() {
            match &self.assets[id] {
                Asset::MeshData(rc) => Some(rc.clone()),
                _ => None,
            }
        }
        else{
            None
        }
        
    }

    pub fn get_material_with_index(&self, id: usize) -> Option<Rc<RefCell<Material>>> {
        if id < self.assets.len() {
            match &self.assets[id] {
                Asset::Material(rc) => Some(rc.clone()),
                _ => None,
            }
        }
        else{
            None
        }
    }

    pub fn get_material_instance_with_index(&self, id: usize) -> Option<Rc<RefCell<MaterialInstance>>> {
        if id < self.assets.len() {
            match &self.assets[id] {
                Asset::MaterialInstance(rc) => Some(rc.clone()),
                _ => None,
            }
        }
        else{
            None
        }
    }

    pub fn get_texture_with_index(&self, id: usize) -> Option<Rc<WebGlTexture>> {
        if id < self.assets.len() {
            match &self.assets[id] {
                Asset::Texture(rc) => Some(rc.clone()),
                _ => None,
            }
        }
        else{
            None
        }
    }

    pub fn get_parent_material(&self, material_instance_id: usize) -> Option<Rc<RefCell<Material>>> {
        if let Some(material_instance) = self.get_material_instance_with_index(material_instance_id) {
            Some(material_instance.borrow().get_parent().clone())
        } else {
            None
        }
    }
}
