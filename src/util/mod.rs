//! Module for common utilities

#[cfg(feature = "regexp")]
mod regexp;

#[cfg(feature = "regexp")]
pub use regexp::{RegExp,Matches};