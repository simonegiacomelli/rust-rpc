use std::{convert::Infallible, error::Error, fs, net::SocketAddr};
use std::fs::{create_dir_all, read};
use std::future::Future;
use std::io::{BufRead, Read};
use std::path::{Component, Path, PathBuf};

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper::body::Body;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use rust_rpc::g_result::GResult;
// use crate::read::{self, Fused, Reference};


type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

const blob_store_folder: &str = "./data/blob-store";


async fn web_handler(req: Request<IncomingBody>) -> GResult<Response<BoxBody>> {
    let string = req.uri().to_string();
    let uri = rem_first(string.as_str());
    let buf = Path::new(blob_store_folder).canonicalize()?;
    let abs = buf.to_str().ok_or("err0.9")?;
    let resource_path = resource_resolve(abs, uri).ok_or("err1.0")?;

    let mut rdr = req.collect().await?.aggregate().reader();
    let mut body = Vec::new();
    let body_size = rdr.read_to_end(&mut body)?;
    println!("size={} ", body_size);

    let path_buf = PathBuf::from(resource_path.clone());
    let parent = path_buf.parent().ok_or("err1.1")?;
    create_dir_all(parent)?;

    fs::write(resource_path, body)?;

    // let r = read::IoRead::new(rdr);
    // reader.read_to_string(&mut body);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full("ciao"))?;

    Ok(response)
    // let method_get = || -> Option<Response<Full<Bytes>>>{
    //     Some(Response::new(Full::new(Bytes::from(request.uri().to_string() + "GET "))))
    // };
    // let method_post = || -> Option<Response<Full<Bytes>>> {
    //     let string = request.uri().to_string();
    //     let uri = rem_first_and_last(string.as_str());
    //     let buf = Path::new(blob_store_folder).canonicalize().ok()?;
    //     let abs = buf.to_str()?;
    //     let f = resource_resolve(abs, uri)?;
    //
    //     // let inc = request.into_body();
    //     Some(Response::new(Full::new(Bytes::from(f + " POST\n"))))
    // };
    //
    // let response = match *request.method() {
    //     Method::POST => method_post(),
    //     _ => method_get()
    // };
    // let r2 = match response {
    //     None => { Response::new(Full::new(Bytes::from("sogna\n"))) }
    //     Some(res) => { res }
    // };
    // Ok(r2)
}


#[tokio::main]
async fn main() -> GResult<()> {
    pretty_env_logger::init();

    create_dir_all(blob_store_folder)?;
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
