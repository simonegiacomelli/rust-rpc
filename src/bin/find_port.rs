use std::borrow::BorrowMut;
use std::net::TcpListener;
use std::ops::Range;
use rust_rpc::find_port::find_port_in;

fn main() {
    if let Some(available_port) = find_port_in(1337..1400) {
        println!("port `{}` is available", available_port);
    }
}
