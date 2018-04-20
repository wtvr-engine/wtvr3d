//! # scene
//!
//! A module that implements a tree structure and entity system for the 3D scene

pub use self::transform::{Transform, TransformId};
pub use self::component::{ComponentId,ComponentBehaviour,Component};
use super::math::Vector3;
use self::component::camera::Camera;
use std::mem;
use std::vec::Vec;
use std::boxed::Box;
use std::collections::HashMap;

pub mod transform;

pub mod component;

/// # Scene
/// Scene is an Arena containing a tree. It is the owner of all the children Transforms and components.
pub struct Scene {

    /// list of transforms in the scene, as a flat array of Transforms.
    transforms : Vec<Transform>,

    /// List of the transform array indexes that could be reused after having been deleted from the tree.
    free_transforms : Vec<TransformId>,

    /// List of all components in the scene
    components : HashMap<ComponentId,Component>,

    /// List of components attached to each transform in the scene.
    component_mapping : HashMap<TransformId,Vec<ComponentId>>,

    /// A counter to give each component a unique id
    next_id : usize
}

macro_rules! gen_comp_getters {
    ($($name:ident($ty:ty),$fnname:ident),*) => {
        $(pub fn $fnname(&mut self, cid : ComponentId) -> &mut Box<$ty> { match self.components.get_mut(&cid) { Some(&mut Component::$name(ref mut x)) => x, _ => panic!() }} )*
    }
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
            components : HashMap::new(),
            component_mapping : HashMap::new(),
            next_id : 0
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut scene = Scene::new();
    /// let tid1 = scene.append_new(None);
    /// assert_eq!(tid1.index,0);
    /// assert_eq!(scene.transforms.len(),1);
    /// let tid2 = scene.append_new(Some(tid1));
    /// assert_eq!(tid2.index,1);
    /// ```
    pub fn append_new(&mut self, parent : Option<TransformId>) -> TransformId {
        let mut t = Transform::new(Vector3::zero(),Vector3::zero(),Vector3 { x: 1.0, y : 1.0, z : 1.0});
        t.parent = parent;
        let mut result = TransformId {index : self.transforms.len() };
        if !self.free_transforms.is_empty() {
            let i = self.free_transforms[0];
            result = i
        }
        if let Some(parent_id) = parent {
            let last_child : Option<TransformId>;
            let first_child : Option<TransformId>;
            {
                let mut parent_transform = self.get_mut(parent_id);
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
            let i = self.free_transforms.remove(0);
            mem::replace(&mut self.transforms[i.index],t);
        }
        else{
            self.transforms.push(t);
        }
        result
    }

    /// Destroys a transform with all its children, its components, and its children's components recursively.
    pub fn destroy(&mut self, tid : TransformId) {

        //destroying and removing current components
        if let Some(comps) = self.component_mapping.get(&tid){
            for cid in comps.iter() {
                if let Some(comp) = self.components.get_mut(&cid){
                    comp.destroy();
                }
                self.components.remove(&cid);
            }
        }
        self.component_mapping.remove(&tid);
        let (psib,nsib,parent,mut next_child) =
        {
            let t = self.get_mut(tid);
            t.set_dead();
            (t.previous_sibling,  t.next_sibling, t.parent,  t.first_child)
        };
        self.free_transforms.push(tid);
        while let Some(next_tid) = next_child {
            self.destroy(next_tid);
            let t2 = self.get(next_tid);
            next_child = t2.next_sibling;
        }
        if let Some(tid2) = psib {
            let t2 = self.get_mut(tid2);
            t2.next_sibling = nsib;
        }
        if let Some(tid2) = nsib {
            let t2 = self.get_mut(tid2);
            t2.previous_sibling = psib;
        }
        if let Some(tid2) = parent {
            let t2 = self.get_mut(tid2);
            if Some(tid) == t2.first_child {
                t2.first_child = nsib;
            }
            if Some(tid) == t2.last_child {
                t2.last_child = psib;
            }
        }
    }

    /// Adds a component to the component list and parents it to a transform, calling initialize in the process.
    pub fn add_component(&mut self, mut comp : Component, parent_id : TransformId) -> ComponentId {
        comp.set_parent(parent_id);
        let cid = ComponentId {index : self.next_id};
        comp.initialize();
        self.components.insert(cid,comp);
        self.next_id += 1;
        let mut insert_new = true;
        if let Some(vec) = self.component_mapping.get_mut(&parent_id){
            vec.push(cid);
            insert_new = false;
        }
        if insert_new {
            let mut comps = Vec::new();
            comps.push(cid);
            self.component_mapping.insert(parent_id,comps);
        }
        cid
    }

    /// Completely destroys and remove a component from the scene, calling destroy() on the component in the process
    pub fn remove_component(&mut self, cid : ComponentId){
        let mut parent_id = None;
        if let Some(comp) = self.components.get_mut(&cid){
            parent_id = comp.get_parent();
            comp.destroy();
        }
        self.components.remove(&cid);
        if let Some(tid) = parent_id {
            if let Some(vec) = self.component_mapping.get_mut(&tid) {
                for i in 0..vec.len() {
                    if vec[i] == cid {
                        vec.remove(i);
                    }
                }
            }
        }
    }

    gen_comp_getters!(Camera(Camera), get_camera, Any(ComponentBehaviour), get_any);

}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_complex_scene() -> Scene {
        let mut scene = Scene::new();
        let t1 = scene.append_new(None);
        scene.get_mut(t1).get_position_mut().x = 2.0;
        let t2 = scene.append_new(Some(t1));
        scene.get_mut(t2).get_scale_mut().y = 3.0;
        let t3 = scene.append_new(Some(t1));
        scene.get_mut(t3).get_rotation_mut().z = -1.5;
        scene.append_new(Some(t3));
        scene.append_new(Some(t3));
        assert_eq!(scene.transforms.len(),5);
        {
            let t2 = scene.get_mut(TransformId {index : 2});
            assert_eq!(t2.first_child,Some(TransformId {index : 3}));
            assert_eq!(t2.last_child,Some(TransformId {index : 4}));
        }
        {
            let t3 = scene.get_mut(TransformId {index : 3});
            assert_eq!(t3.next_sibling,Some(TransformId {index : 4}));
        }
        scene
    }

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
        let t2 = scene.get_mut(TransformId { index : 0});
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

    #[test]
    fn destroy() {
        let mut scene = create_complex_scene();
        {
            let t1 = scene.get_mut(TransformId {index : 1});
            assert_eq!(t1.next_sibling,Some(TransformId {index : 2}));
        }
        scene.destroy(TransformId {index : 2});
        {
            let t1 = scene.get_mut(TransformId {index : 1});
            assert_eq!(t1.next_sibling,None);
        }
        {
            let t0 = scene.get_mut(TransformId {index : 0});
            assert_eq!(t0.first_child, Some(TransformId{ index : 1 }));
            assert_eq!(t0.last_child, Some(TransformId{ index : 1 }));
        }
        {
            let t3 = scene.get_mut(TransformId {index : 3});
            assert_eq!(t3.is_dead(), true);
        }
        {
            let t4 = scene.get_mut(TransformId {index : 4});
            assert_eq!(t4.is_dead(), true);
        }
        {
            let t2 = scene.get_mut(TransformId {index : 2});
            assert_eq!(t2.is_dead(), true);
        }
    }

    #[test]
    fn destroy_then_replace(){
        let mut scene = create_complex_scene();
        scene.destroy(TransformId {index : 2});
        assert_eq!(scene.free_transforms.len(),3);
        let tid2 = scene.append_new(Some(TransformId {index : 1}));
        assert_eq!(scene.free_transforms.len(),2);
        assert_eq!(tid2,TransformId {index : 2});
        let tid3 = scene.append_new(Some(TransformId {index : 1}));
        assert_eq!(scene.free_transforms.len(),1);
        assert_eq!(tid3,TransformId {index : 3});
        let tid4 = scene.append_new(Some(TransformId {index : 3}));
        let tid5 = scene.append_new(Some(TransformId {index : 3}));
        assert_eq!(scene.free_transforms.len(),0);
        assert_eq!(tid4,TransformId {index : 4});
        assert_eq!(tid5,TransformId {index : 5});
        {
            let t1 = scene.get_mut(TransformId {index : 1});
            assert_eq!(t1.first_child, Some(TransformId {index : 2}));
            assert_eq!(t1.last_child, Some(TransformId {index : 3}));
        }
    }

    #[test]
    fn add_component_and_destroy_parent(){
        let mut scene = create_complex_scene();
        let mut cam = Camera::new(200.0,16.0/9.0,10.0,500.0);
        let tid = TransformId {index : 2};
        let cid = scene.add_component(Component::Camera(Box::new(cam)),tid);
        {
            let mut cambox = scene.get_camera(cid);
            if let Some(pid) = cambox.get_parent() {
                assert_eq!(pid.index,2);
            }
            else{
                panic!("Camera does not have a parent!");
            }
        }
        {
            if let Some(vec) = scene.component_mapping.get_mut(&tid) {
                assert_eq!(vec.len(),1);
                assert_eq!(vec[0].index,cid.index);
            }
            else{
                panic!("The components aren't well registered in compnent_mapping");
            }
        }
        {
            scene.destroy(tid);
        }
        {
            if let Some(vec) = scene.component_mapping.get(&tid){
                panic!("Component mapping hasn't been removed properly for transform!");
            }
            if let Some(x) = scene.components.get(&cid){
                panic!("Component has not been removed from the scene.");
            }
        }
    }

    #[test]
    fn remove_component(){
        let mut scene = create_complex_scene();
        let mut cam = Camera::new(200.0,16.0/9.0,10.0,500.0);
        let tid = TransformId {index : 2};
        let cid = scene.add_component(Component::Camera(Box::new(cam)),tid);
        scene.remove_component(cid);
        {
            if let Some(vec) = scene.component_mapping.get(&tid) {
                assert_eq!(vec.len(),0);
            }
            else{
                panic!("The component has not been removed from component_mapping");
            }
            if let Some(x) = scene.components.get(&cid){
                panic!("The component has not been removed from the scene");
            }
        }
    }

}
