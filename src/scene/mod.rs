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

#[wasm_bindgen]
pub struct Scene {
    main_renderer: Option<Renderer>,
    world: World,
}

#[wasm_bindgen]
impl Scene {
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
            &Point3::new(position.x, position.y, position.z),
            &Point3::new(target.x, target.y, target.z),
        );
        let entity = self.world.create_entity().with(camera).build();
        entity.id()
    }
}

impl Scene {
    fn register_components(&mut self) -> () {
        self.world.register::<Transform>();
        self.world.register::<TransformParent>();
        self.world.register::<Camera>();
        self.world.register::<MeshComponent>();
    }
}
