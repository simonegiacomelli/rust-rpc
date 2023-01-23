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

use crate::webserver::{HttpRequest, HttpResponse};

// use crate::read::{self, Fused, Reference};


type GenericError = Box<dyn Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

pub async fn to_http_request(req: Request<Incoming>) -> Option<HttpRequest> {
    let content_type = get_content_type(&req);
    let method = req.method().to_string();
    let url = req.uri().to_string();
    let collected = req.collect().await.ok()?;
    let mut rdr = collected.aggregate().reader();
    // let mut body = Vec::new();
    let mut content = String::new();
    let body_size = rdr.read_to_string(&mut content).ok()?;
    let http_request = HttpRequest {
        method,
        content,
        content_type,
        url,
        parameters: HashMap::new(),
        headers: HashMap::new(),
    };
    Some(http_request)
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