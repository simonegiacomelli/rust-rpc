use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::rpc::{get_handler_key, Payload, Request};

pub struct Proxy {
    transport: Box<dyn Transport>,
}

pub trait Transport {
    fn send(&self, payload: &str) -> String;
}

impl Proxy {
    pub fn new(transport: impl Transport + 'static) -> Proxy {
        Proxy {
            transport: Box::new(transport)
        }
    }
    pub fn send<Req, Res>(&self, req: &Req) -> Res
        where Req: Request<Res>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {
        let handler_key = get_handler_key::<Req, Res>();
        let req_json = serde_json::to_string(req).unwrap();
        let payload = Payload { handler_key: handler_key.as_str(), json: req_json.as_str() };
        let req_payload = payload.to_string();
        let res_payload = self.transport.send(&req_payload);
        let res: Res = serde_json::from_str(&res_payload).unwrap();
        res
    }
}