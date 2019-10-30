//! Scene structure and main wasm-bindgen export
//! The scene has an udpate function to be called each frame.
//! Under the hood, it uses `specs` to work.

use crate::component::{Camera, Mesh, Transform, TransformParent};
use crate::renderer::Renderer;
use crate::utils::console_error;
use crate::utils::transfer_types::Vector3Data;
use specs::{Builder, Entities, ReadStorage, World, WorldExt};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

/// Scene representation, to be shared with JS.
/// A scene holds a renderer and a `specs` world.
#[wasm_bindgen]
pub struct Scene {
    /// The main renderer for the scene  
    /// Is None by default, before being initialized with a Camera.
    main_renderer: Option<Rc<RefCell<Renderer>>>,

    /// The current `specs` World for this scene.
    world: World,
}

#[wasm_bindgen]
pub enum FileType {
    WMesh = 1,
    WMaterial = 2,
    WMatInstance = 3,
}

#[wasm_bindgen]
impl Scene {
    /// Constructor. Initializes a new `Scene` with a fresh world and registers common components.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene {
        let world = World::new();
        let mut scene = Scene {
            main_renderer: None,
            world: world,
        };
        scene.register_components();
        scene
    }

    /// Creates an entity holding a Camera. Returns its Entity ID.
    pub fn create_camera_entity(
        &mut self,
        aspect_ratio: f32,
        fov: f32,
        znear: f32,
        zfar: f32,
        position: Vector3Data,
        target: Vector3Data,
    ) -> u32 {
        let camera = Camera::new(
            aspect_ratio,
            fov,
            znear,
            zfar,
            &position.to_point3(),
            &target.to_point3(),
        );
        let entity = self.world.create_entity().with(camera).build();
        entity.id()
    }

    pub fn create_mesh_entity(&mut self, mesh_data_id: &str, material_instance_id: &str) -> u32 {
        if let Some(renderer) = &self.main_renderer {
            let parent_material_id = renderer
                .borrow()
                .get_asset_registry()
                .get_parent_material_id(material_instance_id);
            if let Some(parent_id) = parent_material_id {
                let mesh = Mesh::new(mesh_data_id, material_instance_id, &parent_id);
                let entity = self.world.create_entity().with(mesh).build();
                entity.id()
            } else {
                console_error("Provided material instance could not be found in registry. Did you forget to register it?");
                u32::max_value()
            }
        } else {
            u32::max_value()
        }
    }

    pub fn register_asset(&mut self, file_data: &[u8], file_type: FileType) -> String {
        match &mut self.main_renderer {
            None => {
                console_error("Trying to register asset before initializing renderer!");
                String::new()
            }
            Some(renderer) => match renderer.borrow_mut().register_asset(file_data, file_type) {
                Err(message) => {
                    console_error(&message);
                    String::new()
                }
                Ok(id) => id,
            },
        }
    }

    /// Initializes the renderer for this Scene. This might fail if no valid camera is supplied.
    pub fn initialize_renderer(
        &mut self,
        canvas: HtmlCanvasElement,
        context: WebGlRenderingContext,
        camera_entity: u32,
    ) -> () {
        if let Some(_) = self.main_renderer {
            return;
        }
        let camera_opt = self.get_camera_for_rendering(camera_entity);
        match camera_opt {
            Err(message) => {
                console_error(message.clone().as_str());
                panic!(message)
            }
            Ok(camera) => {
                self.main_renderer = Some(Rc::new(RefCell::new(Renderer::new(
                    camera, canvas, context,
                ))));
            }
        }
    }

    /// Function to be called each frame.
    pub fn update(&mut self) -> () {
        use specs::DispatcherBuilder;

        if let Some(renderer) = &mut self.main_renderer {
            renderer.borrow_mut().resize_canvas();
            let render_system = crate::system::RenderingSystem::new(renderer.clone());
            let mut dispatcher = DispatcherBuilder::new()
                .with_thread_local(render_system)
                .build();
            dispatcher.dispatch(&self.world);
        } else {
            console_error("Trying to update before initializing the renderer!");
        }
    }
}

impl Scene {
    /// Registers every common component for the current world.
    fn register_components(&mut self) -> () {
        self.world.register::<Transform>();
        self.world.register::<TransformParent>();
        self.world.register::<Camera>();
        self.world.register::<Mesh>();
    }

    /// Gets a camera from the system storage and clones it to pass it to the renderer.  
    /// This might fail if an incorrect ID is given.
    fn get_camera_for_rendering(&self, camera_entity_id: u32) -> Result<Camera, String> {
        let system_data: (ReadStorage<Camera>, Entities) = self.world.system_data();
        let entity = system_data.1.entity(camera_entity_id);
        if let Some(camera) = system_data.0.get(entity) {
            Ok(camera.clone())
        } else {
            Err(String::from("Could not find the requested Camera."))
        }
    }
}
