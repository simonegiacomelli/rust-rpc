use wasm_bindgen::prelude::*;

use hello_common::MulRequest;
use rpc_api::rpc::rpc_version;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert(&get_string());
}

pub fn get_string() -> String {
    format!("rpc-lib version {}", rpc_version())
}

fn ciccio() {
    MulRequest { a: 20, b: 22 };
}
