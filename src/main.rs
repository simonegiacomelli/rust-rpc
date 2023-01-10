use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rust_rpc::ContextHandler;
use rust_rpc::response::{PointRequest, PointResponse};



fn main() {
    let point = PointRequest { x: 1, y: 2 };
    let result = serde_json::to_string(&point).unwrap();
    println!("json=`{}`", result);
    let mut context_handler = ContextHandler::new();
    context_handler.register(|p: PointRequest| -> PointResponse {
        PointResponse { sum: p.x + p.y }
        // Foo { sum: p.x + p.y }
    });

    // let aftermath = context_handler.dispatch2(
    //     "rust_rpc::PointRequest-rust_rpc::PointResponse".to_owned(),
    //     r#"{"x":1,"y":2}"#.to_owned(),
    // );

    // println!("aftermath={aftermath}");

    // let result = proxy.send(PointRequest(3,8));
}



#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    sum: i32,
}
