//! Scene structure and main wasm-bindgen export
//! The scene has an udpate function to be called each frame.
//! Under the hood, it uses `specs` to work.

use crate::component::{Camera, Enabled, MeshComponent, Transform, TransformParent};
use crate::renderer::Renderer;
use crate::utils::console_error;
use crate::utils::transfer_types::Vector3Data;
use specs::{Builder, Entities, ReadStorage, World, WorldExt};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

/// Scene representation, to be shared with JS.
/// A scene holds a renderer and a `specs` world.
#[wasm_bindgen]
pub struct Scene {
    /// The main renderer for the scene  
    /// Is None by default, before being initialized with a Camera.
    main_renderer: Option<Renderer>,

    /// The current `specs` World for this scene.
    world: World,
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
        let entity = self
            .world
            .create_entity()
            .with(camera)
            .with(Enabled::default())
            .build();
        entity.id()
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
                self.main_renderer = Some(Renderer::new(camera, canvas, context));
            }
        }
    }

    pub fn render(&mut self) -> () {
        if let Some(renderer) = &mut self.main_renderer {
            renderer.render_objects();
        }
        else {
            console_error("Trying to render before initializing the renderer!");
        }
    }
}

impl Scene {
    /// Registers every common component for the current world.
    fn register_components(&mut self) -> () {
        self.world.register::<Transform>();
        self.world.register::<TransformParent>();
        self.world.register::<Camera>();
        self.world.register::<MeshComponent>();
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
