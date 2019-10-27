use crate::component::{Camera, MeshComponent, Transform, TransformParent};
/// Scene structure and main wasm-bindgen export
/// The scene has an udpate function to be called each frame.
/// Under the hood, it uses `specs` to work.
use crate::renderer::Renderer;
use crate::utils::transfer_types::Vector3Data;
use nalgebra::Point3;
use specs::{Builder, World, WorldExt};
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
        let entity = self.world.create_entity().with(camera).build();
        entity.id()
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
}
