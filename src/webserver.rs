use std::collections::HashMap;
use std::iter::Map;

pub mod tokio_conversion;
pub mod tokio_server;
pub mod wait_webserver;
pub mod reqwest_transport;


pub struct HttpRequest {
    pub method: String,
    pub content: String,
    pub content_type: String,
    pub url: String,
    pub parameters: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}


pub struct HttpResponse {
    pub content: String,
    pub content_type: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
}


pub struct Context {}


pub type HttpHandler = fn(HttpRequest, Context) -> HttpResponse;