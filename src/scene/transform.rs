//! # transform
//! A Transform is a node in the Scene tree. It allows moving into the tree in any direction.

use super::super::math::{Matrix4,Vector3};
use std::option::Option;

/// # Transform
/// A Transform is a node in the Scene tree. It allows moving into the tree in any direction.
pub struct Transform {

    /// Position of the transform relative to its parent.
    translation : Vector3,

    /// Rotation of the transform relative to its parent.
    rotation : Vector3,

    /// Scale of the transform relative to its parent.
    scale : Vector3,

    /// State of the matrix field. If marked dirty, then the matrix is not in sync with translation, rotation and scale.
    dirty : bool,

    /// A Transformed may be marked dead if it has been destroyed but remains in the `transforms` vec so that its index may be reused.
    dead : bool,

    /// Transform matrix : calculated from translation, rotation and scale if the matrix is marked as dirty.
    matrix : Matrix4,

    /// Parent transform. If None, it's at the root of the scene.
    pub parent : Option<TransformId>,

    /// First child of the transform for easy traversal. The child's parent must link back to this instance.
    pub first_child : Option<TransformId>,

    /// Last child of the transform for easy traversal. The child's parent must link back to this instance.
    pub last_child : Option<TransformId>,

    /// Next sibling of this transform. None if last transform of the children chain.
    pub next_sibling : Option<TransformId>,

    /// Previous sibling of this transform. None if first transform of the children chain.
    pub previous_sibling : Option<TransformId>
}

/// # TransformId
/// A type checked id to reference the transforms in an idiomatic way.
#[derive(Hash,Copy, Clone, PartialEq, Eq)]
pub struct TransformId {
    pub index : usize
}

impl Transform {
    /// Creates a new transform from a relative  translation, rotation and scale. Its matrix won't match by default.
    pub fn new(t: Vector3, r : Vector3, s : Vector3) -> Transform {
         Transform {
            translation : t,
            rotation : r,
            scale : s,
            dirty : true,
            dead: false,
            matrix : Matrix4::identity(),
            parent : None,
            first_child : None,
            last_child : None,
            next_sibling : None,
            previous_sibling : None
        }
    }

    /// Returns the translation field as an immutable vector.
    pub fn get_position(&self) -> &Vector3 {
        &self.translation
    }

    /// Returns the rotation field as an immutable vector.
    pub fn get_rotation(&self) -> &Vector3 {
        &self.rotation
    }

    /// Returns the scale field as an immutable vector.
    pub fn get_scale(&self) -> &Vector3 {
        &self.scale
    }

    /// Returns the translation field as an mutable vector to allow changes.
    pub fn get_position_mut(&mut self) -> &mut Vector3 {
        &mut self.translation
    }

    /// Returns the rotation field as an mutable vector to allow changes.
    pub fn get_rotation_mut(&mut self) -> &mut Vector3 {
        &mut self.rotation
    }

    /// Returns the scale field as an mutable vector to allow changes.
    pub fn get_scale_mut(&mut self) -> &mut Vector3 {
        &mut self.scale
    }

    /// Sets the transform to a dead state so that it is ignored when processing living transforms.
    pub fn set_dead(&mut self) {
        self.dead = true;
    }

    /// Get whether this transform is dead.
    pub fn get_dead(&self) -> bool {
        self.dead
    }

}
