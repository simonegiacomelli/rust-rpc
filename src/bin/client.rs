use rust_rpc::proxy::{Proxy, Transport};
use rust_rpc::rpc::api_point::{PointRequest};

struct HttpTransport {}

impl Transport for HttpTransport {
    fn send(&self, payload: &str) -> String {
        todo!()
    }
}

fn main() {
    let http_transport = HttpTransport {};
    let proxy = Proxy::new(http_transport);
    let request = PointRequest { x: 7, y: 8 };
    let response = proxy.send(&request);
    println!("response = {:?}", response)
}