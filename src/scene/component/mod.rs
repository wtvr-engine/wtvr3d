//! # component
//! Component trait: while Transform holds the 3D position and scene tree information, Components hold the logic and actual object data.
//! This module also defines the different component types.

pub mod camera;

pub use self::camera::Camera;

use super::transform::TransformId;

/// # Component
/// A component attaches to a Transform and gives it functionality. It handles the life cycle of a scene object.
pub trait Component{

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
