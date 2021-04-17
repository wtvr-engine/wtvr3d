//! Error types for WTVR3d

use std::fmt::Display;

use wasm_bindgen::JsValue;
/// WTVR3D error type. Holds a reason and 2 slots for optionnal source and description.
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
        if let Some(desc) = &self.description {
            f.write_str(", Description : ")?;
            f.write_str(desc)?;
        }
        Ok(())
    }
}

impl From<W3DError> for JsValue {
    fn from(e: W3DError) -> Self {
        JsValue::from_str(&e.to_string())
    }
}
