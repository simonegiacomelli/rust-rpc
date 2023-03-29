use std::fmt::Debug;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::rpc;
use crate::rpc::{conversions, get_handler_key, ReqResBound};
use crate::rpc::handlers::{Payload, Request};

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
    pub async fn send<Req: ReqResBound, Res: ReqResBound>(&self, req: &Req) -> Result<Res, String>
        where Req: Request<Res>,
    {
        let req_payload = self.request_to_payload(req);
        let res_payload = self.transport.send(&req_payload).await;
        let res = conversions::rpc_res_from_str(&res_payload);
        res
    }

    fn request_to_payload<Req: ReqResBound>(&self, req: &Req) -> String {
        let handler_key = get_handler_key::<Req>();
        let req_json = conversions::rpc_req_to_str(req);
        let payload = Payload { handler_key: handler_key.as_str(), json: req_json.as_str() };
        let req_payload = payload.to_string();
        req_payload
    }
}
