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
use rust_rpc::webserver::HttpResponse;
use rust_rpc::webserver::tokio_server::webserver_start;


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;


#[tokio::main]
async fn main() -> Result<()> {
    webserver_start( "",
        |req, ctx| -> HttpResponse {
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


