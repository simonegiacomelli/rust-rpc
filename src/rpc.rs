use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::g_result::GResult;

pub mod api_point;
mod context_handler;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}


pub trait Request<Req> {}


pub struct ContextHandler {
    handlers: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

pub fn get_handler_key<Req, Res>() -> String
    where Req: Request<Res>,
{
    let req_name = std::any::type_name::<Req>();
    let res_name = std::any::type_name::<Res>();

    let key = format!("{req_name}-{res_name}");
    key
}


pub struct Payload<'a> {
    pub handler_key: &'a str,
    pub json: &'a str,
}

impl<'a> Payload<'a> {
    pub fn from(payload: &str) -> Payload {
        let mut s = payload.splitn(2, "\n");
        let h = s.next().unwrap();
        let j = s.next().unwrap();
        Payload {
            handler_key: h,
            json: j,
        }
    }

    pub fn to_string(&self) -> String {
        self.handler_key.to_owned() + "\n" + &*self.json
    }
}


fn rpc_req_from_str<Req>(payload: &str) -> Req
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let req: Req = serde_json::from_str(payload).unwrap();
    req
}

fn rpc_res_to_str<Res>(res: &Res) -> String
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let res_json = serde_json::to_string(&res).unwrap();
    res_json
}

pub fn rpc_req_to_str<Req>(req: &Req) -> String
    where Req: ?Sized + Serialize + DeserializeOwned + Debug {
    let req_json = serde_json::to_string(req).unwrap();
    req_json
}

pub fn rpc_res_from_str<Res>(res_payload: &String) -> Res
    where Res: ?Sized + Serialize + DeserializeOwned + Debug {
    let res: Res = serde_json::from_str(&res_payload).unwrap();
    res
}
