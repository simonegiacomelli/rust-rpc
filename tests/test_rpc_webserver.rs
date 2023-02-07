extern crate core;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use rust_rpc::*;
use rust_rpc::find_port::find_port;
use rust_rpc::rpc::handlers::{Handlers, Request};
use rust_rpc::rpc::Proxy;
use rust_rpc::webserver::HttpResponse;
use rust_rpc::webserver::reqwest_transport::HttpReqwestTransport;
use rust_rpc::webserver::tokio_server::webserver_start;
use rust_rpc::webserver::wait_webserver::wait_webserver_responsive;

#[tokio::test]
async fn test_no_context() {
    let port = find_port().unwrap();
    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start(&string, |req| -> HttpResponse {
            // if req.method == "GET" { return HttpResponse::new2("GET method not supported"); }
            // TODO spostare handler fuori / oppure altra soluzione?
            let mut context_handler = Handlers::<()>::new();
            context_handler.register(move |req: MulRequest, _| -> MulResponse {
                MulResponse { mulResult: req.a * req.b }
            });
            let res = context_handler.dispatch(&req.content, ());
            HttpResponse::new(res)
        }).await.unwrap();
    });
    let url = &format!("http://127.0.0.1:{}", port);
    wait_webserver_responsive(url).await;


    let url = url.to_string();
    let http_transport = HttpReqwestTransport { url };
    let proxy = Proxy::new(http_transport);

    let request = MulRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await.unwrap();
    assert_eq!(response.mulResult, 42);

    let request = AddRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await;
    assert!(response.is_err());
    assert!(response.err().unwrap().contains("handler not found"));
}

#[tokio::test]
async fn test_with_context() {
    let port = find_port().unwrap();
    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start(&string, |req| -> HttpResponse {
            // if req.method == "GET" { return HttpResponse::new2("GET method not supported"); }
            // TODO spostare handler fuori / oppure altra soluzione?
            let mut context_handler = Handlers::<String>::new();
            context_handler.register(move |req: MulRequest, ctx: String| -> MulResponse {
                assert_eq!(ctx, "context1");
                MulResponse { mulResult: req.a * req.b }
            });
            let res = context_handler.dispatch(&req.content, "context1".to_string());
            HttpResponse::new(res)
        }).await.unwrap();
    });
    let url = &format!("http://127.0.0.1:{}", port);
    wait_webserver_responsive(url).await;


    let url = url.to_string();
    let http_transport = HttpReqwestTransport { url };
    let proxy = Proxy::new(http_transport);

    let request = MulRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await.unwrap();
    assert_eq!(response.mulResult, 42);

    let request = AddRequest { a: 6, b: 7 };
    let response = proxy.send(&request).await;
    assert!(response.is_err());
    assert!(response.err().unwrap().contains("handler not found"));
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

