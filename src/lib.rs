extern crate js_sys;
extern crate web_sys;

mod utils;
mod ws;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(unused_imports)]
use js_sys::Object;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}
