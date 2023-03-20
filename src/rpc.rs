pub use proxy::Proxy;
pub use proxy::Transport;

pub mod api_point;
pub mod handlers;
pub mod conversions;
mod proxy;

fn get_handler_key<Req>() -> String { std::any::type_name::<Req>().to_string() }