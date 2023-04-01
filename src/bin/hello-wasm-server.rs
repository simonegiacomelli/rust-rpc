use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use rpc_api::rpc::handlers::{Handlers, Request};
use rust_rpc::g_result::GResult;
use rust_rpc::webserver::tokio_server::webserver_start_arc;

static context1: &str = "context1";

#[tokio::main]
async fn main() {
    println!("hello-wasm-server");
    let mut handlers = Handlers::<String>::new();
    handlers.register(|req: MulRequest, ctx: String| -> Result<MulResponse, String> {
        assert_eq!(ctx, context1);
        Ok(MulResponse { mulResult: req.a * req.b })
    });

    let http_handler = handlers.new_http_handler(|req| { context1.to_string() });

    let host_port = "0.0.0.0:6666";
    tokio::spawn(async move { webserver_start_arc(&host_port, http_handler).await.unwrap(); });

    loop { sleep(Duration::from_millis(10000)).await; }
}


impl Request<MulResponse> for MulRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MulRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MulResponse {
    pub mulResult: i32,
}
