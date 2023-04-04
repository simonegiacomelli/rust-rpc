use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use trait_set::trait_set;

pub use proxy::Proxy;
pub use proxy::Transport;

pub mod handlers;
pub mod conversions;
pub mod http;
mod proxy;
pub mod reqwest_transport;
pub mod properties;

fn get_handler_key<Req>() -> String { std::any::type_name::<Req>().to_string() }

trait_set! {
    pub trait ReqResBound = Serialize + DeserializeOwned + Debug ;
}

pub fn rpc_version() -> String { "0.0.1".to_string() }
