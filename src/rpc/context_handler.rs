use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::rpc;
use crate::rpc::{ContextHandler, Payload, Request};

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
        let handler_key = rpc::get_handler_key::<Req, Res>();
        println!("registering=`{handler_key}`");
        self.handlers.insert(handler_key, Box::new(move |payload| {
            // TODO centralizzare la ser/des in modo che gestisca una variante Err(str)/Ok<T>(t:T)

            let req = rpc::rpc_req_from_str(payload);
            let res = callback(req);
            let res_json = rpc::rpc_res_to_str(&res);
            res_json
        }));
    }


    pub fn dispatch(&self, request_payload: &str) -> String {
        let p = Payload::from(request_payload);
        let x = self.handlers.get(p.handler_key).unwrap();
        x(p.json)
    }
}
