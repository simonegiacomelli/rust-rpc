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
use rust_rpc::webserver::{HttpRequest, HttpResponse};
use rust_rpc::webserver::tokio::to_http_request;
// use crate::read::{self, Fused, Reference};


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

const blob_store_folder: &str = "./data/blob-store";

type HttpHandler = fn(HttpRequest, Context) -> HttpResponse;


async fn web_handler(callback: HttpHandler, req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    let http_request = to_http_request(req).await?;
    let http_response = callback(http_request, Context {});

    let response = to_http_response(http_response);

    response
}

fn to_http_response(http_response: HttpResponse) -> Result<Response<BoxBody>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, http_response.content_type)
        .body(full(http_response.content))?;
    Ok(response)
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

struct Context {}

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


fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    // chars.next_back();
    chars.as_str()
}


pub fn resource_resolve(root: &str, child: &str) -> Option<String> {
    let root_path = Path::new(root);
    let result = root_path.join(child).to_lexical_absolute().ok()?;
    if !result.starts_with(root_path) {
        return None;
    }
    Some(result.to_str()?.to_string())
}

trait LexicalAbsolute {
    fn to_lexical_absolute(&self) -> std::io::Result<PathBuf>;
}

impl LexicalAbsolute for Path {
    fn to_lexical_absolute(&self) -> std::io::Result<PathBuf> {
        let mut absolute = if self.is_absolute() {
            PathBuf::new()
        } else {
            std::env::current_dir()?
        };
        for component in self.components() {
            match component {
                Component::CurDir => {}
                Component::ParentDir => { absolute.pop(); }
                component @ _ => absolute.push(component.as_os_str()),
            }
        }
        Ok(absolute)
    }
}


