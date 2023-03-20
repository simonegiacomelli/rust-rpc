extern crate core;

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use rust_rpc::*;
use rust_rpc::find_port::find_port;
use rust_rpc::rpc::handlers::{Handlers, Request};
use rust_rpc::rpc::Proxy;
use rust_rpc::webserver::{HttpHandler, HttpRequest, HttpResponse};
use rust_rpc::webserver::reqwest_transport::HttpReqwestTransport;
use rust_rpc::webserver::tokio_server::webserver_start_arc;
use rust_rpc::webserver::wait_webserver::wait_webserver_responsive;

struct TcpPort {
    port: u16,
}

impl TcpPort {
    fn new() -> TcpPort { TcpPort { port: find_port().unwrap() } }
    fn host_port(&self) -> String { format!("127.0.0.1:{}", self.port) }
    fn url(&self) -> String { format!("http://{}", self.host_port()) }
}

#[tokio::test]
async fn test_no_context() {
    let mut context_handler = Handlers::<()>::new();
    context_handler.register(move |req: MulRequest, _| -> Result<MulResponse, String> {
        Ok(MulResponse { mulResult: req.a * req.b })
    });
    context_handler.register(move |req: DivRequest, _| -> Result<DivResponse, String> {
        Err("error1".to_string())
    });

    let callback = new_p(context_handler);

    let tcp_port = TcpPort::new();
    let host_port = tcp_port.host_port();

    tokio::spawn(async move {
        webserver_start_arc(&host_port, callback).await.unwrap();
    });

    wait_webserver_responsive(&tcp_port.url()).await;

    let http_transport = HttpReqwestTransport { url: tcp_port.url() };
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

fn new_p(mut context_handler: Handlers<()>) -> HttpHandler {
    let callback = Arc::new(move |req: HttpRequest| -> HttpResponse {
        let res = context_handler.dispatch(&req.content, ());
        HttpResponse::new(res)
    });
    callback
}

static context1: &str = "context1";

#[tokio::test]
async fn test_with_context() {
    let mut context_handler = Handlers::<String>::new();
    context_handler.register(|req: MulRequest, ctx: String| -> Result<MulResponse, String> {
        assert_eq!(ctx, context1);
        Ok(MulResponse { mulResult: req.a * req.b })
    });

    let callback = context_handler.http_adapter(|req| { context1.to_string() });

    let tcp_port = TcpPort::new();
    let host_port = tcp_port.host_port();
    tokio::spawn(async move {
        webserver_start_arc(&host_port, callback).await.unwrap();
    });

    wait_webserver_responsive(&tcp_port.url()).await;


    let http_transport = HttpReqwestTransport { url: tcp_port.url() };
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

