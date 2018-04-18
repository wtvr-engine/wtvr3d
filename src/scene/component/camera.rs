//! # camera
//! A camera component

use super::super::super::math::{Matrix4,Vector3};
use super::super::transform::{Transform, TransformId};
use super::{ComponentBehaviour};

pub struct Camera {
    projection : Matrix4,
    parent : Option<TransformId>,
    fov : f32,
    aspect_ratio : f32,
    near_z : f32,
    far_z : f32,
    active : bool,
    dirty : bool
}

impl Camera {

    /// Creates a new Camera component from a fov, aspect ration, near and far plane.
    pub fn new(fov : f32, aspect_ratio : f32, nearz : f32, farz : f32) -> Camera {
        Camera {
            fov : fov,
            aspect_ratio: aspect_ratio,
            near_z : nearz,
            far_z : farz,
            projection : Matrix4::perspective(fov,aspect_ratio,nearz,farz),
            parent : None,
            active : false,
            dirty : false
        }
    }

    pub fn set_fov(&mut self, fov : f32){
        self.fov = fov;
        self.dirty = true;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio : f32) {
        self.aspect_ratio = aspect_ratio;
        self.dirty = true;
    }

    pub fn set_near_z(&mut self, nearz : f32) {
        self.near_z = nearz;
        self.dirty = true;
    }

    pub fn set_far_z(&mut self, farz : f32) {
        self.far_z = farz;
        self.dirty = true;
    }

    fn recalculate_matrix(&mut self){
        self.projection = Matrix4::perspective(self.fov,self.aspect_ratio,self.near_z,self.far_z);
        self.dirty = false;
    }
}

impl ComponentBehaviour for Camera {

    fn get_parent(&self) -> Option<TransformId> {
        self.parent
    }

    fn set_parent(&mut self, tid : TransformId){
        self.parent = Some(tid);
    }

    fn enable(&mut self) {
        self.active = true;
    }

    fn disable(&mut self) {
        self.active = false;
    }

    fn update(&mut self) {
        if !self.active {return;}

        if self.dirty {
            self.recalculate_matrix();
        }
    }
}
