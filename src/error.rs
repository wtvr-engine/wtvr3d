//! Error types for WTVR3d

use std::fmt::Display;
/// WTVR3D error
#[derive(Debug, Clone)]
pub struct W3DError {
    reason: &'static str,
    pub source: Option<String>,
    pub description: Option<String>,
}

impl W3DError {
    pub fn new(reason: &'static str, source: Option<String>) -> W3DError {
        W3DError {
            reason,
            source,
            description: None,
        }
    }
    pub fn new_with_desc(
        reason: &'static str,
        source: Option<String>,
        description: Option<String>,
    ) -> W3DError {
        W3DError {
            reason,
            source,
            description,
        }
    }
}

impl Display for W3DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.reason)?;
        if let Some(source) = &self.source {
            f.write_str(", Generated for : ")?;
            f.write_str(source)?;
        }
        Ok(())
    }
}
