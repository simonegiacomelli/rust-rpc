// use std::{convert::Infallible, error::Error, fs, net::SocketAddr};
// use std::fs::{create_dir_all, read};
// use std::future::Future;
// use std::io::{BufRead, Read};
// use std::path::{Path, PathBuf};
//
// use bytes::{Buf, Bytes};
// use http_body_util::{BodyExt, Full};
// use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
// use hyper::body::Body;
// use hyper::server::conn::http1;
// use hyper::service::service_fn;
// use tokio::net::TcpListener;
// // use crate::read::{self, Fused, Reference};
// use rust_blob_api::resource_resolve;
//
// type GenericError = Box<dyn std::error::Error + Send + Sync>;
// type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
// type Result<T> = std::result::Result<T, GenericError>;
// async fn web_handler(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
//       let response = Response::builder()
//         .status(StatusCode::OK)
//         .header(header::CONTENT_TYPE, "application/json")
//         .body(full("ciao"))?;
//
// }
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     pretty_env_logger::init();
//
//     create_dir_all(blob_store_folder)?;
//     let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();
//     println!("serving on http://{}", addr);
//     let listener = TcpListener::bind(&addr).await?;
//     loop {
//         let (stream, _) = listener.accept().await?;
//
//         tokio::task::spawn(async move {
//             let service = service_fn(move |req| web_handler(req));
//
//             if let Err(err) = http1::Builder::new()
//                 .serve_connection(stream, service)
//                 .await
//             {
//                 println!("Failed to serve connection: {:?}", err);
//             }
//         });
//     }
// }
