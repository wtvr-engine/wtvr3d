//! # component
//! Component trait: while Transform holds the 3D position and scene tree information, Components hold the logic and actual object data.
//! This module also defines the different component types.

pub mod camera;

pub use self::camera::Camera;

use super::transform::TransformId;
use std::collections::HashMap;

/// # ComponentBehaviour
/// A component attaches to a Transform and gives it functionality. It handles the life cycle of a scene object.
/// The component behaviour trait defines the functionnality necessary for each component.
pub trait ComponentBehaviour{

    /// Returns the parent Transform of the component.
    fn get_parent(&self) -> Option<TransformId>;

    /// Sets the current parent of the component
    fn set_parent(&mut self, tid : TransformId) -> ();

    /// Function executed  when the component is appended to its parent transform.
    fn initialize(&mut self) -> () {}

    /// Function to enable this component
    fn enable(&mut self) -> () {}

    /// Function executed at the start of the first frame.
    fn start(&mut self) -> () {}

    /// Function executed each frame.
    fn update(&mut self) -> () {}

    /// Function to disable this component.
    fn disable(&mut self) -> () {}

    /// Function executed before destroying the component.
    fn destroy(&mut self) -> () {}
}

/// # ComponentId
/// A typed id for components.
#[derive(Hash,Copy, Clone, PartialEq, Eq, Debug)]
pub struct ComponentId {
    pub index : usize
}

macro_rules! gen_comp_enum {
    ($name:ident; $($var:ident($ty:ty)),*) => {
        /// # Component
        /// Component is an enum for differenciating the different component types.
        /// Component implements the ComponentBehaviour trait, like any of its variants, and each variant must implement the ComponentBehaviour trait.
        pub enum $name {
            $(
                $var($ty),
            )*
        }

        impl ComponentBehaviour for $name {
            fn get_parent(&self) -> Option<TransformId> { match self { $(&$name::$var(ref x) => x.get_parent(),)*}}
            fn set_parent(&mut self, tid : TransformId) -> () { match self { $(&mut $name::$var(ref mut x) => x.set_parent(tid),)*}}
            fn initialize(&mut self) -> () { match self { $(&mut $name::$var(ref mut x) => x.initialize(),)*}}
            fn enable(&mut self) -> () { match self { $(&mut $name::$var(ref mut x) => x.enable(),)*}}
            fn start(&mut self) -> () {match self { $(&mut $name::$var(ref mut x) => x.start(),)*}}
            fn update(&mut self) -> () {match self { $(&mut $name::$var(ref mut x) => x.update(),)*}}
            fn disable(&mut self) -> () {match self { $(&mut $name::$var(ref mut x) => x.disable(),)*}}
            fn destroy(&mut self) -> () {match self { $(&mut $name::$var(ref mut x) => x.destroy(),)*}}
        }
    }
}

gen_comp_enum! (Component; Camera(Box<Camera>),Any(Box<ComponentBehaviour>));
