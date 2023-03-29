use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use proxy::Proxy;
pub use proxy::Transport;
use trait_set::trait_set;

pub mod api_point;
pub mod handlers;
pub mod conversions;
mod proxy;

fn get_handler_key<Req>() -> String { std::any::type_name::<Req>().to_string() }

trait_set! {
    pub trait ReqResBound = Serialize + DeserializeOwned + Debug ;
}
