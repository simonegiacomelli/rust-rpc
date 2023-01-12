use std::{convert::Infallible, error::Error, fs, net::SocketAddr};
use std::fs::{create_dir_all, read};
use std::future::Future;
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper::body::{Body, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use rust_rpc::webserver::{HttpRequest, HttpResponse};
// use crate::read::{self, Fused, Reference};
// use rust_blob_api::resource_resolve;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

async fn route_handler(req: HttpRequest) -> HttpResponse {
    HttpResponse {
        content_type: "text/plain".to_string(),
        content: "ciao".to_string(),
    }
}

async fn web_handler(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    let content_type = get_content_type(&req);
    let method = req.method().to_string();
    let mut rdr = req.collect().await?.aggregate().reader();
    // let mut body = Vec::new();
    let mut content = String::new();
    let body_size = rdr.read_to_string(&mut content)?;
    let http_request = HttpRequest {
        content,
        content_type,
        method,
    };
    let http_response = route_handler(http_request).await;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(full("ciao"))?;
    // let response = Response::builder()
    //     .status(StatusCode::OK)
    //     .header(header::CONTENT_TYPE, http_response.content_type)
    //     .body(full(http_response.content))?;
    Ok(response)
}

fn get_content_type(req: &Request<Incoming>) -> String {
    let ct = req.headers();
    let header_name = "Content-Type";
    if ct.contains_key(header_name) {
        ct.get(header_name).unwrap().to_str().unwrap().to_string()
    } else {
        "".to_string()
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();
    println!("serving on http://{}", addr);
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| web_handler(req));

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
