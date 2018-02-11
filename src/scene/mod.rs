use super::math::{Matrix4,Vector3};
use std::option::Option;
use std::boxed::Box;

pub struct SceneObject {
    translation : Vector3,
    rotation : Vector3,
    scale : Vector3,
    dirty : bool,
    matrix : Matrix4,
    parent : Option<Box<SceneObject>>,
    first_child : Option<Box<SceneObject>>,
    last_child : Option<Box<SceneObject>>,
    next_sibling : Option<Box<SceneObject>>,
    previous_sibling : Option<Box<SceneObject>>
}

impl SceneObject{
    pub fn new(t: Vector3, r : Vector3, s : Vector3) -> SceneObject {
         SceneObject {
            translation : t,
            rotation : r,
            scale : s,
            dirty : true,
            matrix : Matrix4::identity(),
            parent : None,
            first_child : None,
            last_child : None,
            next_sibling : None,
            previous_sibling : None
        }
    }
}
