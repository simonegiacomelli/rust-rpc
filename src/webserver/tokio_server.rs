extern crate core;

use std::{convert::Infallible, error::Error, fs, net::SocketAddr};
use std::collections::HashMap;
use std::fs::{create_dir_all, read};
use std::future::Future;
use std::io::{BufRead, Read};
use std::iter::Map;
use std::ops::Add;
use std::path::{Component, Path, PathBuf};

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper::body::{Body, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;

use rust_rpc::webserver;
use rust_rpc::webserver::{HttpRequest, HttpResponse};
use rust_rpc::webserver::tokio_conversion::to_http_request;

// use crate::read::{self, Fused, Reference};


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

const blob_store_folder: &str = "./data/blob-store";

type HttpHandler = fn(HttpRequest, Context) -> HttpResponse;

#[tokio::main]
async fn main() -> Result<()> {
    webserver_start(|req, ctx| -> HttpResponse {
        println!(" hello !");
        let content = format!("<h1>from a fn, url: {}</h1>", req.url);

        HttpResponse {
            content,
            content_type: "text/html".to_string(),
            status: 200,
            headers: HashMap::new(),
        }
    }).await.unwrap();
    Ok(())
}


async fn webserver_start(callback: HttpHandler) -> Result<()> {
    println!("webserver_start");
    let service = service_fn(move |req| web_handler(callback, req));

    pretty_env_logger::init();

    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();
    println!("serving on http://{}", addr);
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}


async fn web_handler(callback: HttpHandler, req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    let http_request = to_http_request(req).await?;
    let http_response = callback(http_request, Context {});
    webserver::tokio_conversion::to_http_response(http_response)
}

struct Context {}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::error::Error;
    use std::thread;
    use std::thread::Thread;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    use rust_rpc::webserver::HttpResponse;

    use crate::webserver_start;

    #[tokio::test]
    async fn test_in_root() {
        tokio::spawn(async {
            webserver_start(|req, ctx| -> HttpResponse {
                println!(" hello !");
                let content = format!("<h1>from a fn, url: {}</h1>", req.url);

                HttpResponse {
                    content,
                    content_type: "text/html".to_string(),
                    status: 200,
                    headers: HashMap::new(),
                }
            }).await.unwrap();
        });

        wait_webserver_responsive("http://127.0.0.1:1337").await;
        tokio::time::sleep(Duration::new(1, 0)).await;
        reqwest::get("http://127.0.0.1:1337").await.unwrap().text().await;
        tokio::time::sleep(Duration::new(1, 0)).await;
        // std::thread::sleep(Duration::new(5, 0));
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[tokio::test]
    async fn test_wait() {
        tokio::spawn(async {
            webserver_start(|req, ctx| -> HttpResponse {
                println!(" hello !");
                let content = format!("<h1>from a fn, url: {}</h1>", req.url);

                HttpResponse {
                    content,
                    content_type: "text/html".to_string(),
                    status: 404,
                    headers: HashMap::new(),
                }
            }).await.unwrap();
        });

        wait_webserver_responsive("http://127.0.0.1:4444").await;
        tokio::time::sleep(Duration::new(1, 0)).await;
        reqwest::get("http://127.0.0.1:1337").await.unwrap().text().await;
        tokio::time::sleep(Duration::new(1, 0)).await;
        // std::thread::sleep(Duration::new(5, 0));
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    async fn async_function(msg: &str) {
        println!("{}", msg);
    }

    async fn wait_webserver_responsive(url: &str) {
        for _ in 0..300 {
            let res = reqwest::get(url).await;
            println!("{:?}", res);
            match res {
                Ok(ok) => { return; }
                Err(err) => {
                    println!("is_connect={}", err.is_connect());
                }
            }

            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        panic!("timeout waiting for {}",url);
    }
}

