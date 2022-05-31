use std::net::SocketAddrV4;

use crate::connection::Connection;

pub struct Client {
    conn: Connection,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse::<SocketAddrV4>().unwrap();
        Client {
            conn: Connection::new(addr),
        }
    }
}
