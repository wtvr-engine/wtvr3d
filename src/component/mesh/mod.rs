//! Representation of a mesh in a scene

pub mod mesh_data;

use crate::renderer::buffer::Buffer;
use crate::renderer::material::MaterialInstance;
pub use mesh_data::MeshData;
use specs::{Component, VecStorage};
use std::rc::Rc;
use web_sys::WebGlRenderingContext;

/// Typedef for `MeshID` to ensure correct handling of `Mesh` ids with type checking.
pub type MeshID = u32;

/// Mesh component for an entity in the 3D scene.  
/// Links some `MeshData` to some `MaterialInstance`.
pub struct Mesh {
    /// `MeshData` in use for this mesh, containing the vertex data.  
    /// Several meshes can share the same mesh data, hence the `Rc` reference.
    data: Rc<MeshData>,

    /// `MaterialInstance` to use to render this mesh in the 3d scene
    pub material: MaterialInstance,

    /// Identifier for the mesh. Since mesh is linked to JsValue, `Mesh` can't
    /// be a specs component and needs a referencing ID. It will be automatically
    /// created when registering the `Mesh` in the `Renderer`'s repository.
    id: Option<MeshID>,
}

impl Mesh {
    /// Constructor. Uses a `MeshData` instance and a `MaterialInstance`
    pub fn new(data: MeshData, material: MaterialInstance) -> Mesh {
        Mesh {
            data: Rc::new(data),
            material: material,
            id: None,
        }
    }

    /// Returns a slice of the associated `WebGlProgram` buffers
    pub fn get_buffers(&self) -> &[Buffer] {
        self.data.get_buffers()
    }

    /// Returns the number of vertices in this `Mesh`e's `Buffer`s.
    pub fn get_vertex_count(&self) -> i32 {
        self.data.get_vertex_count()
    }

    /// Lookup for this Mesh attributes locations, going though every underlying buffer.  
    /// Should be done at initialization time before any rendering starts.
    pub fn lookup_locations(&mut self, context: &WebGlRenderingContext) -> () {
        for buffer in self.get_buffers() {
            self.material
                .get_parent()
                .borrow_mut()
                .register_new_attribute(buffer.get_attribute_name().to_owned())
        }
        self.material.lookup_locations(context);
    }

    /// Getter for this `Mesh`'s ID.
    pub fn get_id(&self) -> Option<MeshID> {
        self.id
    }

    /// Getter for the private `id` attribute. If it does'nt exist yet, it is supplied by
    /// the caller and set directly.  
    /// This is only meant to be used by the `Renderer`
    pub fn get_or_set_id(&mut self, default: MeshID) -> MeshID {
        if let Some(id) = self.id {
            id
        } else {
            self.id = Some(default);
            default
        }
    }
}

/// MeshComponent is a Mesh reference used to reference an actual mesh in
/// a specs system.  
/// It is used to find associated Meshes when rendering needs to start.
pub struct MeshComponent {
    pub mesh_id: MeshID,
}

impl Component for MeshComponent {
    type Storage = VecStorage<Self>;
}
