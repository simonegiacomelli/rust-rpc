use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use async_trait::async_trait;
use crate::rpc;
use crate::rpc::{get_handler_key, Payload, Request};

pub struct Proxy {
    transport: Box<dyn Transport>,
}


#[async_trait]
pub trait Transport {
    async fn send(&self, payload: &str) -> String;
}

impl Proxy {
    pub fn new(transport: impl Transport + 'static) -> Proxy {
        Proxy {
            transport: Box::new(transport)
        }
    }
    pub async fn send<Req, Res>(&self, req: &Req) -> Res
        where Req: Request<Res>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {
        let handler_key = get_handler_key::<Req, Res>();
        let req_json = rpc::rpc_req_to_str(req);
        let payload = Payload { handler_key: handler_key.as_str(), json: req_json.as_str() };
        let req_payload = payload.to_string();
        let res_payload = self.transport.send(&req_payload).await;
        println!("res_payload={}", res_payload);
        let res = rpc::rpc_res_from_str(&res_payload);
        res
    }
}
