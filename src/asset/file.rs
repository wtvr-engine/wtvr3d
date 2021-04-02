//! The `file` module contains serialization and deserialization for assets

use serde::{Deserialize, Serialize};

use crate::error::W3DError;

/// The file trait is used to serialize and deserialize types efficiently using bincode.
pub trait File<'a>: Serialize + Deserialize<'a> {

    /// Gets the name of the object to serialize for error handling
    fn get_name(&self) -> String;

    /// Serializes the object. This requires the `export` feature to be enabled.
    #[cfg(feature = "export")]
    fn to_file(&self) -> Result<Vec<u8>, W3DError> {
        bincode::serialize(&self)
            .map_err(|_| W3DError::new("Could not serialize object", Some(self.get_name())))
    }

    /// De-serializes a file to an asset object, using bincode.
    fn from_file(data: &'a [u8]) -> Result<Self, W3DError> {
        bincode::deserialize(data).map_err(|_| W3DError::new("Could not deserialize data", None))
    }
}
