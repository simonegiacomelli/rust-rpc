use std::collections::HashMap;
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

#[tokio::test]
async fn test() {
    let port = find_port().unwrap();
    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start(&string, |req, ctx| -> HttpResponse {
            HttpResponse {
                content: "no content".to_string(),
                content_type: "text/html".to_string(),
                status: 404,
                headers: HashMap::new(),
            }
        }).await.unwrap();
    });
    let url = &format!("http://127.0.0.1:{}", port);
    wait_webserver_responsive(url).await;


    let mut context_handler = ContextHandler::new();
    context_handler.register(move |req: MulRequest| -> MulResponse {
        MulResponse { mulResult: req.a * req.b }
    });
    context_handler.register(move |req: AddRequest| -> AddResponse {
        AddResponse { addResult: req.a + req.b }
    });

    let http_transport = MockTransport { context_handler };
    let proxy = Proxy::new(http_transport);

    let request = MulRequest { a: 6, b: 7 };
    let response = proxy.send(&request);
    assert_eq!(response.mulResult, 42);

    let request = AddRequest { a: 6, b: 7 };
    let response = proxy.send(&request);
    assert_eq!(response.addResult, 13)
}

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rust_rpc::find_port::find_port;
use rust_rpc::rpc::ContextHandler;
use rust_rpc::webserver::HttpResponse;
use rust_rpc::webserver::tokio_server::webserver_start;
use rust_rpc::webserver::wait_webserver::wait_webserver_responsive;

use crate::rpc::Request;

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


impl Request<AddResponse> for AddRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddResponse {
    pub addResult: i32,
}

