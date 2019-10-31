//! Representation of a transform in a scene

use nalgebra::{Isometry3, Matrix4, Translation3, UnitQuaternion, Vector3};
use specs::{Component, DenseVecStorage, Entity, FlaggedStorage, NullStorage, VecStorage};
use specs_hierarchy::Parent;

pub struct Transform {
    /// Translation in local space.
    local_translation: Translation3<f32>,

    /// Rotation in local space.
    local_rotation: UnitQuaternion<f32>,

    /// Scale in local space.
    local_scale: Vector3<f32>,

    /// Transform matrix in world space. Needs to be recomputed
    /// if `local_matrix` has changed, along with world matrix for
    /// all of this transform's children.
    world_matrix: Matrix4<f32>,

    /// `true` if `local_matrix` has changed and `world_matrix` needs to be re-computed.
    dirty: bool,
}

impl Transform {
    /// Constructor. Creates a new Transform from a translation, rotation and scale.
    pub fn new(
        translation: &Vector3<f32>,
        rotation: &Vector3<f32>,
        scale: &Vector3<f32>,
    ) -> Transform {
        Transform {
            local_translation: Translation3::from(translation.clone()),
            local_rotation: UnitQuaternion::from_euler_angles(rotation.x, rotation.y, rotation.z),
            local_scale: scale.clone(),
            world_matrix: Matrix4::identity(),
            dirty: true,
        }
    }

    /// Sets a new local translation for this Transform
    pub fn set_translation(&mut self, new_translation: &Vector3<f32>) -> () {
        self.local_translation = Translation3::from(new_translation.clone());
        self.dirty = true;
    }

    /// Sets a new local rotation for this Transform
    pub fn set_rotation(&mut self, new_rotation: &Vector3<f32>) -> () {
        self.local_rotation =
            UnitQuaternion::from_euler_angles(new_rotation.x, new_rotation.y, new_rotation.z);
        self.dirty = true;
    }

    /// Sets a new local scale for this Transform
    pub fn set_scale(&mut self, new_scale: &Vector3<f32>) -> () {
        self.local_scale = new_scale.clone();
        self.dirty = true;
    }

    /// Re-computes world matrix from its inner properties and a given parent world matrix.
    pub fn refresh_world_matrix(&mut self, parent_world_matrix: Option<Matrix4<f32>>) -> () {
        let scale_matrix = Matrix4::new_nonuniform_scaling(&self.local_scale);
        let isometry =
            Isometry3::from_parts(self.local_translation.clone(), self.local_rotation.clone());
        let local_matrix = isometry.to_homogeneous() * scale_matrix;
        if let Some(parent_matrix) = parent_world_matrix {
            self.world_matrix = parent_matrix * local_matrix;
        } else {
            self.world_matrix = local_matrix;
        }
        self.dirty = false;
    }

    /// Getter for the world matrix
    pub fn get_world_matrix(&self) -> Result<Matrix4<f32>, String> {
        if self.dirty {
            crate::utils::console_error("Dirty transform fetch!");
            Err(String::from(
                "Trying to get world matrix while it is dirty!",
            ))
        } else {
            Ok(self.world_matrix)
        }
    }
}

impl Component for Transform {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// Component that represents a parent-child relationship between entities to help build a Scene-graph
pub struct TransformParent {
    /// Represents the parent Entity of the other Entity to which this TransformParent is attached.
    entity: Entity,
}

impl Component for TransformParent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Parent for TransformParent {
    fn parent_entity(&self) -> Entity {
        self.entity
    }
}

/// The Enabled component is a flag component stating that the object should be updated and rendered.
#[derive(Default)]
pub struct Enabled;

#[derive(Default)]
pub struct DirtyTransform;

impl Component for Enabled {
    type Storage = NullStorage<Self>;
}

impl Component for DirtyTransform {
    type Storage = NullStorage<Self>;
}
