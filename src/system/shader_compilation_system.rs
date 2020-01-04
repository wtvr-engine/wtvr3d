use crate::component::Mesh;
use crate::renderer::{LightConfiguration, Renderer};
use crate::utils::console_error;
use specs::{Join, Read, ReadStorage, System};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ShaderCompilationSystem {
    renderer: Rc<RefCell<Renderer>>,
}

impl ShaderCompilationSystem {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> ShaderCompilationSystem {
        ShaderCompilationSystem { renderer: renderer }
    }
}

impl<'a> System<'a> for ShaderCompilationSystem {
    type SystemData = (ReadStorage<'a, Mesh>, Read<'a, LightConfiguration>);
    fn run(&mut self, (mesh, light_config): Self::SystemData) {
        for mesh in (&mesh).join() {
            match mesh.compile_material(self.renderer.clone(), &light_config) {
                Err(message) => console_error(&message),
                _ => {}
            }
        }
    }
}
