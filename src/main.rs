use std::collections::HashMap;
use std::fmt::{Debug, Display, format, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json::Deserializer;
use serde_json::Value::String;

trait Response<Res> {}

#[derive(Serialize, Deserialize, Debug)]
struct PointRequest {
    x: i32,
    y: i32,
}

impl Response<PointRequest> for PointResponse {}

#[derive(Serialize, Deserialize, Debug)]
struct PointResponse {
    sum: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    sum: i32,
}


impl Display for PointRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn point_consumer(point: &PointRequest) {}

struct ContextHandler {
    handlers: HashMap<std::string::String, Box<dyn Fn(&str) -> std::string::String>>,
}

impl ContextHandler {
    fn new() -> ContextHandler {
        ContextHandler { handlers: HashMap::new() }
    }
}

impl ContextHandler {
    fn materialize<'a, T>(&mut self, value: &'a str) -> impl Fn() -> T + 'a
        where
            T: ?Sized + Deserialize<'a> + Debug + Display, {
        let l = || -> T {
            let instance: T = serde_json::from_str(value).unwrap();
            let name = std::any::type_name::<T>();
            println!("inside lambda type name {} {}", name, instance);
            return instance;
        };
        return l;
    }
    fn register<'a, Req, Res>(&mut self, callback: impl Fn(Req) -> Res)
        where Res: Response<Req>,
              Req: ?Sized + Serialize + Deserialize<'a> + Debug + 'a,
              Res: ?Sized + Serialize + Deserialize<'a> + Debug + 'a,
    {
        let req_name = std::any::type_name::<Req>();
        let res_name = std::any::type_name::<Res>();
        println!("req_name={req_name}");
        println!("res_name={res_name}");

        let l = |payload: &'a str| -> std::string::String {
            let req: Req = serde_json::from_str(payload).unwrap();
            // let res = callback(req);
            // let res_json = serde_json::to_string(&res).unwrap();
            let res_json = std::string::String::new();
            return res_json;
        };
        let key = format!("{req_name}-{res_name}");
        let bo = Box::new(l);
        // self.handlers.insert(key, bo);
    }

    fn dispatch(&self, key: &str, payload: &str) -> std::string::String {
        return std::string::String::new();
    }
}

fn main() {
    let point = PointRequest { x: 1, y: 2 };

    let mut context_handler = ContextHandler::new();


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    let lambda = context_handler.materialize::<PointRequest>(serialized.as_str());
    lambda();
    context_handler.register(|p: PointRequest| -> PointResponse {
        PointResponse { sum: p.x + p.y }
        // Foo { sum: p.x + p.y }
    });
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: PointRequest = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
