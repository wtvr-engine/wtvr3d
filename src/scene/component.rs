
use super::transform::TransformId;

/// # Component
/// A component attaches to a Transform and gives it functionality. It handles the life cycle of a scene object.
pub trait Component{
    fn id(&self) -> &ComponentId;
    fn parent(&self) -> &TransformId;
    fn initialize(&self) -> () {}
    fn start(&self) -> () {}
    fn update(&self) -> () {}
    fn destroy(&self) -> () {}
}

pub struct ComponentId{
    index : usize
}
