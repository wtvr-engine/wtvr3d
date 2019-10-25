//! # Utils
//!
//! Useful miscelaneous functions

use wasm_bindgen::JsValue;
use web_sys::console::{error_1, log_1, warn_1};

pub fn console_log(message: String) {
    log_1(&JsValue::from_str(&message[..]));
}

pub fn console_warn(message: String) {
    warn_1(&JsValue::from_str(&message[..]));
}

pub fn console_error(message: String) {
    error_1(&JsValue::from_str(&message[..]));
}
