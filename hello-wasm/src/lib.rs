use wasm_bindgen::prelude::*;

use hello_common::MulRequest;
use rpc_api::rpc::{Proxy, rpc_version};
use rpc_api::rpc::reqwest_transport::HttpReqwestTransport;

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

#[wasm_bindgen]
pub fn ciccio2() {}

#[wasm_bindgen]
pub async fn ciccio() {
    MulRequest { a: 20, b: 22 };
    let x = reqwest::Client::new();
    // x.get("http://localhost:6666".to_string()).aw;
    let result = x.get("http://httpbin.org/ip".to_string()).send().await.unwrap();
    let text = result.text().await.unwrap();
    println!("{}", text);
    alert(&text);
}

// #[wasm_bindgen]
pub async fn rpc_mul(a: i32, b: i32) -> i32 {
    let http_transport = HttpReqwestTransport { url: "http://localhost:6666".to_string() };
    let proxy = Proxy::new(http_transport);

    // todo build-and-run.sh da errore sul Transport
    let resp = proxy.send(&MulRequest { a, b }).await;
    resp.unwrap().mulResult
}