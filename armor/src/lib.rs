#![feature(async_closure)]
#![feature(is_some_and)]
#![feature(type_alias_impl_trait)]
#![feature(array_zip)]

mod armory;
mod db;
mod model;
mod scheduler;
mod utils;
mod worker;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, armor!");
}
