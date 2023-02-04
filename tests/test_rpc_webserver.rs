extern crate core;

use std::collections::HashMap;
use rust_rpc::*;


#[tokio::test]
async fn test() {
    let port = find_port().unwrap();
    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start(&string, |req, ctx| -> HttpResponse {
            // if req.method == "GET" { return HttpResponse::new2("GET method not supported"); }
            // TODO spostare handler fuori / oppure altra soluzione?
            let mut context_handler = ContextHandler::new();
            context_handler.register(move |req: MulRequest| -> MulResponse {
                MulResponse { mulResult: req.a * req.b }
            });
            context_handler.register(move |req: AddRequest| -> AddResponse {
                AddResponse { addResult: req.a + req.b }
            });
            let res = context_handler.dispatch(&req.content);
            HttpResponse::new(res)
        }).await.unwrap();
    });
    let url = &format!("http://127.0.0.1:{}", port);
    wait_webserver_responsive(url).await;


    let url = url.to_string();
    let http_transport = HttpReqwestTransport { url };
    let proxy = Proxy::new(http_transport);

    let request = MulRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await;
    assert_eq!(response.mulResult, 42);

    let request = AddRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await;
    assert_eq!(response.addResult, 13);
}

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rust_rpc::find_port::find_port;
use rust_rpc::rpc::context_handler::{ContextHandler, Request};
use rust_rpc::rpc::Proxy;
use rust_rpc::webserver::HttpResponse;
use rust_rpc::webserver::reqwest_transport::HttpReqwestTransport;
use rust_rpc::webserver::tokio_server::webserver_start;
use rust_rpc::webserver::wait_webserver::wait_webserver_responsive;


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

