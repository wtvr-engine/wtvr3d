use super::super::math::{Matrix4,Vector3};
use std::option::Option;

/// # Transform
/// A Transform is a node in the Scene tree. It allows moving into the tree in any direction.
pub struct Transform {
    translation : Vector3,
    rotation : Vector3,
    scale : Vector3,
    dirty : bool,
    enabled : bool,
    dead : bool,
    matrix : Matrix4,
    pub parent : Option<TransformId>,
    pub first_child : Option<TransformId>,
    pub last_child : Option<TransformId>,
    pub next_sibling : Option<TransformId>,
    pub previous_sibling : Option<TransformId>
}

#[derive(Hash,Copy, Clone, PartialEq, Eq)]
pub struct TransformId {
    pub index : usize
}

impl Transform {
    pub fn new(t: Vector3, r : Vector3, s : Vector3) -> Transform {
         Transform {
            translation : t,
            rotation : r,
            scale : s,
            dirty : true,
            enabled : true,
            dead: false,
            matrix : Matrix4::identity(),
            parent : None,
            first_child : None,
            last_child : None,
            next_sibling : None,
            previous_sibling : None
        }
    }
}
