//! The `to_file` module contains serialization and deserialization for assets

use serde::{Deserialize, Serialize};

use crate::error::W3DError;

pub trait File<'a>: Serialize + Deserialize<'a> {
    fn get_name(&self) -> String;

    #[cfg(feature = "export")]
    fn to_file(&self) -> Result<Vec<u8>, W3DError> {
        bincode::serialize(&self)
            .map_err(|_| W3DError::new("Could not serialize object", Some(self.get_name())))
    }

    fn from_file(data: &'a [u8]) -> Result<Self, W3DError> {
        bincode::deserialize(data).map_err(|_| W3DError::new("Could not deserialize data", None))
    }
}
