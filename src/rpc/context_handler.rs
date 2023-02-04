use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::rpc;
use crate::rpc::conversions;
use crate::rpc::conversions::rpc_error;

pub trait Request<Req> {}


pub struct ContextHandler {
    handlers: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

impl ContextHandler {
    pub fn new() -> ContextHandler { ContextHandler { handlers: HashMap::new() } }

    /**
    turning points:
     - https://serde.rs/lifetimes.html vedi DeserializeOwned
     - https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6e7942d56b8d29e9b3f90893bd650bfb
     - e poi una serie di SO che utilizzavano la 'move' sulla lambda
     */
    pub fn register<Req, Res>(&mut self, callback: impl Fn(Req) -> Res + 'static)
        where Req: Request<Res>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {
        let handler_key = get_handler_key::<Req, Res>();
        println!("registering=`{handler_key}`");
        self.handlers.insert(handler_key, Box::new(move |payload| {
            let req = conversions::rpc_req_from_str(payload);
            if let Err(msg) = req { return msg; }
            let res = callback(req.unwrap());
            let res_json = conversions::rpc_res_to_str(&res);
            res_json
        }));
    }


    pub fn dispatch(&self, request_payload: &str) -> String {
        let payload = Payload::from(request_payload);
        match payload {
            Err(msg) => { rpc_error(&msg) }
            Ok(payload) => {
                let x = self.handlers.get(payload.handler_key);
                match x {
                    None => {
                        let msg1 = format!("dispatch handler not found `{}`", payload.handler_key);
                        rpc_error(&msg1)
                    }
                    Some(fun) => { fun(payload.json) }
                }
            }
        }
    }
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

