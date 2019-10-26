//! # module `utils`
//!
//! Useful miscelaneous functions

use wasm_bindgen::JsValue;
use web_sys::console::{error_1, log_1, warn_1};

/// Logs to the console with `log` level.
pub fn console_log(message: &str) {
    log_1(&JsValue::from_str(message));
}

/// Logs to the console with `warn` level.
pub fn console_warn(message: &str) {
    warn_1(&JsValue::from_str(message));
}

/// Logs to the console with `error` level.
pub fn console_error(message: &str) {
    error_1(&JsValue::from_str(message));
}
