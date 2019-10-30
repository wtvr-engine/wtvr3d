use crate::component::{Mesh, Transform};
use crate::renderer::{Renderer, SortedMeshes};
use specs::{Join, ReadStorage, System};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct RenderingSystem {
    renderer: Rc<RefCell<Renderer>>,
}

impl RenderingSystem {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderingSystem {
        RenderingSystem { renderer: renderer }
    }
}

// ⭕ TODO : add an Enabled component to render only relevant meshes
// ⭕ TODO : Only render objects that are in the camera's reach
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Mesh>, ReadStorage<'a, Transform>);
    fn run(&mut self, (mesh, transform): Self::SystemData) {
        let mut sorted_meshes: SortedMeshes = HashMap::new();
        for (mesh, transform) in (&mesh, &transform).join() {
            let material_id = mesh.get_material_id();
            let mesh_data_id = mesh.get_mesh_data_id();
            let mesh_instance_id = mesh.get_material_instance_id();
            if let Some(mesh_hash_map) = sorted_meshes.get_mut(material_id) {
                if let Some(transform_vec) = mesh_hash_map.get_mut(mesh_data_id) {
                    transform_vec.push((mesh_instance_id, &transform));
                } else {
                    mesh_hash_map.insert(mesh_data_id, vec![(mesh_instance_id, &transform)]);
                }
            } else {
                let mut mesh_hash_map = HashMap::new();
                mesh_hash_map.insert(mesh_data_id, vec![(mesh_instance_id, transform)]);
                sorted_meshes.insert(material_id, mesh_hash_map);
            }
        }
        self.renderer.borrow_mut().render_objects(sorted_meshes);
    }
}
