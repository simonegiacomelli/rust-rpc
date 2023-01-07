use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn point_consumer(point: &Point) {}

struct ContextHandler {
    handlers: HashMap<String, String>,
}

impl ContextHandler {
    fn new() -> ContextHandler {
        ContextHandler { handlers: HashMap::new() }
    }
}

impl ContextHandler {
    fn materialize<'a,T>(&mut self, value: &str)
        where
            T: ?Sized + Deserialize<'a> + Debug + Display, {
        let l = || -> () {
            let instance: T = serde_json::from_str(value).unwrap();
            println!("{}",instance);
        };

        // self.handlers.insert("Point")
    }
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let mut context_handler = ContextHandler::new();


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    context_handler.materialize(serialized.as_str());
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
