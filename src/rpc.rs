use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub mod api_point;

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
            let req: Req = serde_json::from_str(payload).unwrap();
            let res = callback(req);
            let res_json = serde_json::to_string(&res).unwrap();
            res_json
        }));
    }

    pub fn dispatch(&self, request_payload: &str) -> String {
        let p = Payload::from(request_payload);
        let x = self.handlers.get(p.handler_key).unwrap();
        x(p.json)
    }
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


