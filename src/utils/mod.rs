//! # Utils
//! 
//! Useful miscelaneous functions

use web_sys::console::{warn_1,log_1,error_1};
use wasm_bindgen::JsValue;

pub fn console_log(message : String){
    log_1(&JsValue::from_str(&message[..]));
}

pub fn console_warn(message : String){
    warn_1(&JsValue::from_str(&message[..]));
}

pub fn console_error(message : String){
    error_1(&JsValue::from_str(&message[..]));
}
