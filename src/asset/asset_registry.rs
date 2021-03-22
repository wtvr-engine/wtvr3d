//! Central asset registration object, associated with an instance of the engine.

use std::{cell::RefCell, rc::Rc};

use crate::asset::Asset;
pub struct AssetRegistry(Vec<Rc<RefCell<Asset>>>);