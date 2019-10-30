use crate::component::{Mesh, Transform};
use crate::renderer::Renderer;
use specs::{Join, ReadStorage, System};
use std::cell::RefCell;
use std::rc::Rc;

pub struct RenderingSystem {
    renderer: Rc<RefCell<Renderer>>,
}

impl RenderingSystem {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderingSystem {
        RenderingSystem { renderer: renderer }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Mesh>, ReadStorage<'a, Transform>);
    fn run(&mut self, (mesh, transform): Self::SystemData) {
        for (mesh, transform) in (&mesh, &transform).join() {
            //Add mesh to a vec, prepare to sort them
        }
    }
}
