
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rpc::Response;

pub mod rpc;


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
        where Res: Response<Req>,
              Req: ?Sized + Serialize + DeserializeOwned + Debug,
              Res: ?Sized + Serialize + DeserializeOwned + Debug,
    {
        let req_name = std::any::type_name::<Req>();
        let res_name = std::any::type_name::<Res>();

        let key = format!("{req_name}-{res_name}");
        println!("registering=`{key}`");
        self.handlers.insert(key, Box::new(move |payload| {
            let req: Req = serde_json::from_str(payload).unwrap();
            let res = callback(req);
            let res_json = serde_json::to_string(&res).unwrap();
            res_json
        }));
    }
    pub fn dispatch(&self, key: &str, payload: &str) -> String {
        let x = self.handlers.get(key).unwrap();
        x(payload)
    }
    pub fn dispatch2(&self, key: String, payload: String) -> String {
        let k = key.as_str();
        let x = self.handlers.get(k).unwrap();
        x(payload.as_str())
    }
}
