use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::task::Context;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::rpc::{conversions, get_handler_key, ReqResBound};
use crate::rpc::conversions::rpc_error;
use crate::rpc::http::{HttpHandler, HttpRequest, HttpResponse};

pub trait Request<Req> {}

pub struct Handlers<Ctx> {
    handlers: HashMap<String, Box<dyn Fn(&str, Ctx) -> String + Send + Sync>>,
}


impl<Ctx: 'static> Handlers<Ctx> {
    pub fn new() -> Handlers<Ctx> {
        Handlers { handlers: HashMap::new() }
    }

    pub fn new_http_handler(self, context_generator: fn(&HttpRequest) -> Ctx) -> HttpHandler {
        Arc::new(move |req: HttpRequest| -> HttpResponse {
            HttpResponse::new(self.dispatch(&req.content, context_generator(&req)))
        })
    }

    pub fn register<Req: ReqResBound, Res: ReqResBound>(&mut self, callback: impl Fn(Req, Ctx) -> Result<Res, String> + Send + Sync + 'static)
        where Req: Request<Res>
    {
        let handler_key = get_handler_key::<Req>();
        println!("registering=`{handler_key}`");
        self.handlers.insert(handler_key, Box::new(move |payload, ctx| {
            let req = conversions::rpc_req_from_str(payload);
            if let Err(msg) = req { return msg; }
            let res = callback(req.unwrap(), ctx);
            match res {
                Ok(ok) => { conversions::rpc_res_to_str(&ok) }
                Err(msg) => { rpc_error(&msg) }
            }
        }));
    }

    pub fn dispatch(&self, request_payload: &str, ctx: Ctx) -> String {
        let payload = Payload::from(request_payload);
        match payload {
            Err(msg) => { rpc_error(&msg) }
            Ok(payload) => {
                let x = self.handlers.get(payload.handler_key);
                match x {
                    None => {
                        let msg1 = format!("dispatch(...) handler not found `{}`", payload.handler_key);
                        rpc_error(&msg1)
                    }
                    Some(fun) => { fun(payload.json, ctx) }
                }
            }
        }
    }
}


pub struct Payload<'a> {
    pub handler_key: &'a str,
    pub json: &'a str,
}

impl<'a> Payload<'a> {
    pub fn from(payload: &str) -> Result<Payload, String> {
        let mut s = payload.splitn(2, "\n");
        let h = s.next().ok_or("Payload split 1")?;
        let j = s.next().ok_or("Payload split 2")?;
        Ok(Payload {
            handler_key: h,
            json: j,
        })
    }

    pub fn to_string(&self) -> String {
        self.handler_key.to_owned() + "\n" + &*self.json
    }
}

