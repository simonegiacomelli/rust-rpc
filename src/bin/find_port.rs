use std::borrow::BorrowMut;
use std::net::TcpListener;
use std::ops::Range;

fn main() {
    if let Some(available_port) = find_port_in(1337..1400) {
        println!("port `{}` is available", available_port);
    }
}

fn find_port() -> Option<u16> {
    find_port_in(10000..20000)
}

fn find_port_in(range: Range<u16>) -> Option<u16> {
    let mut range1 = range.clone();
    range1.find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

