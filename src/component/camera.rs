//! Camera component. Used as the point of vue to render the scene.

use nalgebra::{zero, Isometry3, Matrix4, Perspective3, Point3, Vector3};
use specs::{Component, VecStorage};

/// Represents a Camera in the scene, with its projection data.
/// Might be improved in the future to include orthographic mode.
pub struct Camera {
    /// The projection matrix for this camera
    projection: Perspective3<f32>,

    /// The view matrix for this camera.  
    /// ⚠ Will be removed in favor of a normal transform component for the camera
    // ⭕ TODO : move this in a transform component
    view: Isometry3<f32>,

    /// View Projection matrix for this `Camera`.  
    /// Automatically computed from transform and projection data.
    vp_matrix: Matrix4<f32>,

    /// `true` if position or projection has changed and the `vp_matrix` needs to be re-computed.
    dirty: bool,
}

impl Camera {
    /// Constructor. Needs all projection data and initial position and "look-at" target.
    pub fn new(
        aspect_ratio: f32,
        fov: f32,
        znear: f32,
        zfar: f32,
        position: &Point3<f32>,
        target: &Point3<f32>,
    ) -> Camera {
        let projection = Perspective3::new(aspect_ratio, fov, znear, zfar);
        let view = Isometry3::look_at_rh(position, target, &Vector3::y());
        Camera {
            projection: projection,
            view: view,
            vp_matrix: zero(),
            dirty: true,
        }
    }

    /// Setter for the aspect_ration of this camera. Useful when the viewport size changes.
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> () {
        self.projection.set_aspect(aspect_ratio);
    }

    /// Getter for the view-projection matrix. Returns None if the `vp_matrix` is marked as `dirty`.
    pub fn get_vp_matrix(&self) -> Result<&Matrix4<f32>, String> {
        if self.dirty {
            Err(String::from(
                "Trying to get camera's vp_matrix while it is dirty!",
            ))
        } else {
            Ok(&self.vp_matrix)
        }
    }

    /// Function that computes the view projection matrix on demand from the projection and position matrices.
    pub fn compute_vp_matrix(&mut self) -> &Matrix4<f32> {
        if self.dirty {
            self.vp_matrix = self.projection.into_inner() * self.view.to_homogeneous();
            self.dirty = false;
        }
        &self.vp_matrix
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}
