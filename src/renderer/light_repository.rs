use crate::component::{Cone, Light};
use crate::renderer::{Material, Uniform};
use nalgebra::{Vector3, Vector4};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use web_sys::WebGlRenderingContext;

/// Struct to hold the current light configuration in terms of number of lights of each type
#[derive(Default,PartialEq,Eq,Clone)]
pub struct LightConfiguration {
    pub directional: usize,
    pub point: usize,
    pub spot: usize,
}

/// Resource for sharing light information between the light system and the rendering system
#[derive(Default)]
pub struct LightRepository {
    pub ambiant: Option<Light>,
    pub directional: Vec<(Light, Vector3<f32>)>,
    pub point: Vec<(Light, Vector3<f32>)>,
    pub spot: Vec<(Light, Vector3<f32>, Vector3<f32>, Cone)>,
}

impl LightRepository {
    pub fn set_material_uniforms(
        &self,
        context: &WebGlRenderingContext,
        material: Rc<RefCell<Material>>,
    ) {
        let mat = material.borrow();
        if let Some(light) = &self.ambiant {
            let ambiant_loc = &mat.global_uniform_locations.ambiant_light_location;
            let ambiant_uniform = Uniform::new_with_location(
                "",
                ambiant_loc.clone(),
                Box::new(Vector4::new(
                    light.color.x,
                    light.color.y,
                    light.color.z,
                    light.intensity,
                )),
            );
            ambiant_uniform.set_to_context(context).ok();
        }

        for (i, dir_light) in self.directional.iter().enumerate() {
            LightRepository::set_light_uniform(context, &mat, &dir_light.0, false, dir_light.1, i)
        }
        for (i, point_light) in self.point.iter().enumerate() {
            LightRepository::set_light_uniform(
                context,
                &mat,
                &point_light.0,
                false,
                point_light.1,
                i,
            )
        }
    }

    fn set_light_uniform(
        context: &WebGlRenderingContext,
        material: &Ref<Material>,
        light: &Light,
        point: bool,
        dir_or_pos: Vector3<f32>,
        index: usize,
    ) -> () {
        let locations = if point {
            &material.global_uniform_locations.point_lights_locations
        } else {
            &material
                .global_uniform_locations
                .directional_lights_locations
        };
        let color_uniform = Uniform::new_with_location(
            "",
            locations[index].color.clone(),
            Box::new(Vector3::new(
                light.color.x,
                light.color.y,
                light.color.z,
            )),
        );
        color_uniform.set_to_context(context).ok();
        let intensity_uniform = Uniform::new_with_location(
            "",
            locations[index].intensity.clone(),
            Box::new(light.intensity),
        );
        intensity_uniform.set_to_context(context).ok();
        let attenuation_uniform = Uniform::new_with_location(
            "",
            locations[index].attenuation.clone(),
            Box::new(light.attenuation),
        );
        attenuation_uniform.set_to_context(context).ok();
        let dir_pos_uniform = Uniform::new_with_location(
            "",
            locations[index].position_or_direction.clone(),
            Box::new(dir_or_pos),
        );
        dir_pos_uniform.set_to_context(context).ok();
    }
}
