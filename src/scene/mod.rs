//! # scene
//!
//! A module that implements a tree structure and entity system for the 3D scene

pub use self::transform::{Transform, TransformId};
pub use self::component::{Component};
use super::math::Vector3;
use std::mem;
use std::vec::Vec;
use std::boxed::Box;
use std::collections::HashMap;

pub mod transform;

pub mod component;

/// # Scene
/// Scene is an Arena containing a tree. It is the owner of all the children Transforms and components.
pub struct Scene {
    transforms : Vec<Transform>,
    free_transforms : Vec<TransformId>,
    components : HashMap<TransformId,Vec<Box<Component>>>
}


/// Creates a new empty scene. Usually, you will have one unique scene.
///
/// # Examples
///
/// ```
/// let scene = Scene::new();
/// ```
impl Scene {
    pub fn new() -> Scene {
        Scene {
            transforms : Vec::new(),
            free_transforms : Vec::new(),
            components : HashMap::new()
        }
    }

    pub fn get_mut(&mut self, tid : TransformId) -> &mut Transform {
        &mut self.transforms[tid.index]
    }

    pub fn get(&self, tid : TransformId) -> &Transform {
        &self.transforms[tid.index]
    }

    pub fn append_new(&mut self, parent : Option<TransformId>) -> TransformId {
        let mut t = Transform::new(Vector3::zero(),Vector3::zero(),Vector3 { x: 1.0, y : 1.0, z : 1.0});
        t.parent = parent;
        let mut result = TransformId {index : self.transforms.len() };
        if let Some(parentId) = parent {
            let last_child : Option<TransformId>;
            {
                let mut parentTransform = self.get_mut(parentId);
                t.previous_sibling = parentTransform.last_child;
                parentTransform.last_child = Some(result);
                last_child = t.previous_sibling
            }
            if let Some(lchild) = last_child {
                self.get_mut(lchild).next_sibling = Some(result);
            }

        }
        if !self.free_transforms.is_empty() {
            let i = self.free_transforms[0];
            mem::replace(&mut self.transforms[i.index],t);
            self.free_transforms.remove(0);
            result = i
        }
        else{
            self.transforms.push(t);
        }
        result
    }


}
