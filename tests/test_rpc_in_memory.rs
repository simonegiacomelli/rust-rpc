use rust_rpc::*;
use rust_rpc::proxy::*;

struct MockTransport {
    context_handler: ContextHandler,
}

impl Transport for MockTransport {
    fn send(&self, payload: &str) -> String {
        self.context_handler.dispatch(payload)
    }
}

#[test]
fn test() {
    let mut context_handler = ContextHandler::new();
    context_handler.register(move |req: MulRequest| -> MulResponse {
        MulResponse { result: req.a * req.b }
    });
    let http_transport = MockTransport { context_handler: context_handler };
    let proxy = Proxy::new(http_transport);
    let request = MulRequest { a: 6, b: 7 };
    let response = proxy.send(&request);
    assert_eq!(response.result, 42)
}

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rust_rpc::rpc::ContextHandler;

use crate::rpc::Request;

#[derive(Serialize, Deserialize, Debug)]
pub struct MulRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MulResponse {
    pub result: i32,
}


impl Request<MulResponse> for MulRequest {}
