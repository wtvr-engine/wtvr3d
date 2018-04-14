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



impl Scene {
    /// Creates a new empty scene. Usually, you will have one unique scene.
    ///
    /// # Examples
    ///
    /// ```
    /// let scene = Scene::new();
    /// ```
    pub fn new() -> Scene {
        Scene {
            transforms : Vec::new(),
            free_transforms : Vec::new(),
            components : HashMap::new()
        }
    }

    /// Returns a mutable reference to a transform. You can only hold one mutable transform reference at a time.
    pub fn get_mut(&mut self, tid : TransformId) -> &mut Transform {
        &mut self.transforms[tid.index]
    }

    /// Returns an immutable reference to a transform.
    pub fn get(&self, tid : TransformId) -> &Transform {
        &self.transforms[tid.index]
    }

    /// Appends a new transform to the scene with an optionnal parent, and returns the matching TransformId.
    pub fn append_new(&mut self, parent : Option<TransformId>) -> TransformId {
        let mut t = Transform::new(Vector3::zero(),Vector3::zero(),Vector3 { x: 1.0, y : 1.0, z : 1.0});
        t.parent = parent;
        let mut result = TransformId {index : self.transforms.len() };
        if let Some(parentId) = parent {
            let last_child : Option<TransformId>;
            let first_child : Option<TransformId>;
            {
                let mut parent_transform = self.get_mut(parentId);
                t.previous_sibling = parent_transform.last_child;
                parent_transform.last_child = Some(result);
                last_child = t.previous_sibling;
                first_child = parent_transform.first_child;
                if first_child == None {
                    parent_transform.first_child = Some(result);
                }
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

#[cfg(test)]
mod tests {

    use super::*;
    //use super::math::Vector3;

    #[test]
    fn new() {
        let scene = Scene::new();
        assert_eq!(scene.transforms.len(), 0);
        assert_eq!(scene.free_transforms.len(),0);
        assert_eq!(scene.components.len(),0);
    }

    #[test]
    fn get() {
        let mut scene = Scene::new();
        scene.transforms.push(Transform::new(Vector3::zero(),Vector3::zero(),Vector3{ x: 2.0, y : 1.0, z : 1.0}));
        {
            let t1 = scene.get(TransformId { index : 0});
            assert_eq!(t1.get_scale().x, 2.0);
            assert_eq!(t1.get_position().z, 0.0);
        }
        let mut t2 = scene.get_mut(TransformId { index : 0});
        t2.get_position_mut().z = 2.0;
        assert_eq!(t2.get_position().z,2.0);
    }

    #[test]
    fn append_new() {
        let mut scene = Scene::new();
        let tid1 = scene.append_new(None);
        assert_eq!(tid1.index,0);
        assert_eq!(scene.transforms.len(),1);
        let tid2 = scene.append_new(Some(tid1));
        assert_eq!(tid2.index,1);
        if let Some(tid3) = scene.get(tid2).parent {
            assert_eq!(tid1.index,tid3.index);
            if let Some(tid4) = scene.get(tid3).last_child {
                assert_eq!(tid4.index, tid2.index);
            }
            else{
                panic!();
            }
        }
        else{
            panic!();
        }
    }

}
