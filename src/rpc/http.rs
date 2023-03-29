use std::collections::HashMap;
use std::sync::Arc;

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

impl HttpResponse {
    pub fn new(content: String) -> HttpResponse {
        HttpResponse {
            content,
            content_type: "plain/text".to_string(),
            status: 200,
            headers: HashMap::new(),
        }
    }
    pub fn new2(content: &str) -> HttpResponse {
        HttpResponse {
            content: content.to_string(),
            content_type: "plain/text".to_string(),
            status: 200,
            headers: HashMap::new(),
        }
    }
}


pub type HttpHandler = Arc<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>;
