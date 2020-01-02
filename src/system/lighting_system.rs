//! System for registering lights before rendering

use crate::component::{Light,Direction,Transform,Cone,Enabled};
use crate::renderer::LightRepository;
use nalgebra::{Vector3, Vector4};
use specs::{System,Write,ReadStorage, Join,Entities};



pub struct LightingSystem;

impl<'a> System<'a> for LightingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Direction>,
        ReadStorage<'a, Cone>,
        ReadStorage<'a, Enabled>,
        Write<'a, LightRepository>,
    );
    fn run(&mut self, (entities, lights, transforms, directions, cones, enableds, mut light_repository): Self::SystemData) {
        light_repository.ambiant = None;
        light_repository.directional.clear();
        light_repository.point.clear();
        light_repository.spot.clear();
        let mut ambiant = Light { color : Vector3::new(0.0,0.0,0.0), intensity : 0.0, attenuation : 0.0};
        let mut some_ambiant = false;
        for (entity,light, _) in (&entities, &lights, &enableds).join() {
            let direction_opt = directions.get(entity);
            let transform_opt = transforms.get(entity);
            let cone_opt = cones.get(entity);
            if let (Some(direction),None) = (direction_opt,cone_opt) {
                light_repository.directional.push((light.clone(),direction.0));
            }
            else if let (Some(transform),None,None) = (transform_opt,cone_opt,direction_opt){
                let world_position = transform.get_world_matrix() * Vector4::new(0.0,0.0,0.0,1.0);
                light_repository.point.push((light.clone(),Vector3::new(world_position.x,world_position.y,world_position.z)));
            }
            else if let (Some(direction),Some(cone),Some(transform)) = (direction_opt,cone_opt,transform_opt){
                let world_position = transform.get_world_matrix() * Vector4::new(0.0,0.0,0.0,1.0);
                light_repository.spot.push((light.clone(),Vector3::new(world_position.x,world_position.y,world_position.z),direction.0,cone.clone()));
            }
            else if let (None,None,None) = (transform_opt,cone_opt,direction_opt) {
                some_ambiant = true;
                ambiant.color = ambiant.color * ambiant.intensity + light.color * light.intensity;
                ambiant.intensity = ambiant.intensity + light.intensity;
            }
        }
        if some_ambiant {
            light_repository.ambiant = Some(ambiant);
        }
    }
}