mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;
use log::{ info, warn, Level };

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // Show wasm errors in the browser's console tab
    set_panic_hook();

    // Log level config
    console_log::init_with_level(Level::Debug).unwrap();
    // console_log::init_with_level(Level::Trace).unwrap();

    // wasm bindgem alert
    alert(&format!("Hello, {}!", name));

    // Not available when log level is Error
    warn!("This is a warn message example");

    // Not available when log level is Warn
    info!("There is an info message example");

    log::error!("[error] It works?!");

    // This is the rust panic to be shown in the browser's console
    panic!("Panic test!");
}
