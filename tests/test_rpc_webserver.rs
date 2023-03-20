extern crate core;

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use rust_rpc::*;
use rust_rpc::find_port::find_port;
use rust_rpc::rpc::handlers::{Handlers, Request};
use rust_rpc::rpc::Proxy;
use rust_rpc::webserver::{HttpRequest, HttpResponse};
use rust_rpc::webserver::reqwest_transport::HttpReqwestTransport;
use rust_rpc::webserver::tokio_server::webserver_start_arc;
use rust_rpc::webserver::wait_webserver::wait_webserver_responsive;

#[tokio::test]
async fn test_no_context() {
    let mut context_handler = Handlers::<()>::new();
    context_handler.register(move |req: MulRequest, _| -> Result<MulResponse, String> {
        Ok(MulResponse { mulResult: req.a * req.b })
    });
    context_handler.register(move |req: DivRequest, _| -> Result<DivResponse, String> {
        Err("error1".to_string())
    });

    let callback = move |req: HttpRequest| -> HttpResponse {
        let res = context_handler.dispatch(&req.content, ());
        HttpResponse::new(res)
    };

    let port = find_port().unwrap();
    let host_port = format!("127.0.0.1:{}", port);
    let host_port2 = host_port.clone();

    tokio::spawn(async move {
        webserver_start_arc(&host_port2, Arc::new(callback)).await.unwrap();
    });

    let url = &format!("http://{}", host_port);
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


    let request = DivRequest { a: 1, b: 2 };
    let response = proxy.send(&request).await;
    assert!(response.is_err());
    assert_eq!(response.err().unwrap(), "error1".to_string());
}

static context1: &str = "context1";

#[tokio::test]
async fn test_with_context() {
    let port = find_port().unwrap();

    let callback = move |req: HttpRequest| -> HttpResponse {
        // if req.method == "GET" { return HttpResponse::new2("GET method not supported"); }
        // TODO spostare handler fuori / oppure altra soluzione?
        let mut context_handler = Handlers::<String>::new();
        context_handler.register(|req: MulRequest, ctx: String| -> Result<MulResponse, String> {
            assert_eq!(ctx, context1);
            Ok(MulResponse { mulResult: req.a * req.b })
        });
        let res = context_handler.dispatch(&req.content, context1.to_string());
        HttpResponse::new(res)
    };
    let callback = Arc::new(callback);

    tokio::spawn(async move {
        let string = format!("127.0.0.1:{}", port);
        webserver_start_arc(&string, callback).await.unwrap();
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

pub async fn webserver_start2(callback: impl Fn(HttpRequest) -> HttpResponse) {}

#[tokio::test]
async fn test_with_context2() {
    tokio::spawn(async move {
        webserver_start2(|req| -> HttpResponse {
            let mut context_handler = Handlers::<String>::new();
            context_handler.register(|req: MulRequest, ctx: String| -> Result<MulResponse, String> {
                Err("just testing".to_string())
            });
            let res = context_handler.dispatch(&req.content, context1.to_string());
            HttpResponse::new(res)
        }).await;
    });
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

impl Request<DivResponse> for DivRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DivRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DivResponse {
    pub divResult: i32,
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

