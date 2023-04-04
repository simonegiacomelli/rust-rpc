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

use rpc_api::rpc::http::{HttpRequest, HttpResponse};

// use crate::read::{self, Fused, Reference};


type GenericError = Box<dyn Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

pub async fn to_http_request(req: Request<Incoming>) -> Result<HttpRequest> {
    let content_type = get_content_type(&req);
    let method = req.method().to_string();
    let url = req.uri().to_string();
    let collected = req.collect().await?;
    let mut rdr = collected.aggregate().reader();
    // let mut body = Vec::new();
    let mut content = String::new();
    let _ = rdr.read_to_string(&mut content)?;
    let http_request = HttpRequest {
        method,
        content,
        content_type,
        url,
        parameters: HashMap::new(),
        headers: HashMap::new(),
    };
    Ok(http_request)
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

pub fn to_http_response(http_response: HttpResponse) -> Result<Response<BoxBody>> {
    let mut builder = Response::builder()
        .status(StatusCode::from_u16(http_response.status)?)
        .header(header::CONTENT_TYPE, http_response.content_type);
    for item in  http_response.headers.into_iter(){
        builder = builder.header(item.0,item.1);
    }
    let response = builder
        .body(full(http_response.content))?;
    Ok(response)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
