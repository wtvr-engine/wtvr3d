use crate::component::{DirtyTransform, Enabled, Transform, TransformParent};
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};
use specs_hierarchy::Hierarchy;
use std::collections::HashMap;

pub struct SceneGraphSystem;

impl SceneGraphSystem {
    pub fn new() -> SceneGraphSystem {
        SceneGraphSystem {}
    }
}

impl<'a> System<'a> for SceneGraphSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Hierarchy<TransformParent>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, DirtyTransform>,
        ReadStorage<'a, Enabled>,
    );
    fn run(&mut self, (entities, hierarchy, mut transforms, mut dirty, enabled): Self::SystemData) {
        let mut dirty_transforms = HashMap::new();
        for (entity, _, _, _) in (&entities, &mut transforms, &mut dirty, &enabled).join() {
            let parent_entity_opt = hierarchy.parent(entity);
            if let Some(parent_entity) = parent_entity_opt {
                dirty_transforms.insert(entity, Some(parent_entity));
            }
            else {
                dirty_transforms.insert(entity,None);
            }
            for child in hierarchy.all_children_iter(entity) {
                if !dirty_transforms.contains_key(&child) {
                    if let Some(parent_entity) = hierarchy.parent(child) {
                        dirty_transforms.insert(child, Some(parent_entity));
                    }
                }
            }
        }
        for (entity,parent_entity_opt) in &dirty_transforms {
            if let None = parent_entity_opt {
                transforms.get_mut(*entity).unwrap().refresh_world_matrix(None);
                dirty.remove(*entity);
            }
        }
        for entity in hierarchy.all() {
            if let Some(Some(parent)) = dirty_transforms.get(entity) {
                let mut parent_matrix = None;
                if let Some(parent_transform) = transforms.get(*parent) {
                    parent_matrix = Some(parent_transform.get_world_matrix());
                }
                transforms.get_mut(*entity).unwrap().refresh_world_matrix(parent_matrix);
                dirty.remove(*entity);
            } 
        }
    }
}
