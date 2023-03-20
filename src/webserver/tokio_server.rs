extern crate core;

use std::{convert::Infallible, error::Error, fs, net::SocketAddr};
use std::collections::HashMap;
use std::fs::{create_dir_all, read};
use std::future::Future;
use std::io::{BufRead, Read};
use std::iter::Map;
use std::ops::{Add, Deref};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper::body::{Body, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::webserver;
use crate::webserver::{HttpHandler, HttpRequest, HttpResponse};
use crate::webserver::tokio_conversion::to_http_request;

// use crate::read::{self, Fused, Reference};


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
type Result<T> = std::result::Result<T, GenericError>;

pub async fn webserver_start(host_port: &str, callback: fn(HttpRequest) -> HttpResponse) -> Result<()> {
    webserver_start_arc(host_port, Arc::new(callback)).await
}

pub async fn webserver_start_arc(host_port: &str, callback: HttpHandler) -> Result<()> {
    println!("webserver_start");

    let ignore = pretty_env_logger::try_init();

    let addr: SocketAddr = host_port.parse().unwrap();
    println!("serving on http://{}", addr);
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let callback_clone = Arc::clone(&callback);

        tokio::task::spawn(async move {
            let callback_ref = &callback_clone;
            let service = service_fn(move |req| web_handler(callback_ref, req));

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn web_handler(callback: &HttpHandler, req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    let http_request = to_http_request(req).await?;
    let http_response = callback(http_request);
    webserver::tokio_conversion::to_http_response(http_response)
}



