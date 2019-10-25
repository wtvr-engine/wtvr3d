use nalgebra::{zero, Isometry3, Matrix4, Perspective3, Point3, Vector3};

pub struct Camera {
    projection: Perspective3<f32>,
    view: Isometry3<f32>,
    vp_matrix: Matrix4<f32>,
    dirty: bool,
}

impl Camera {
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

    pub fn get_vp_matrix(&self) -> Option<&Matrix4<f32>> {
        if self.dirty {
            None
        } else {
            Some(&self.vp_matrix)
        }
    }

    pub fn compute_vp_matrix(&mut self) -> &Matrix4<f32> {
        if self.dirty {
            self.vp_matrix = self.projection.into_inner() * self.view.to_homogeneous();
            self.dirty = false;
        }
        &self.vp_matrix
    }
}
