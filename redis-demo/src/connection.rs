use std::{
    fmt::Error,
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
};

pub struct Connection {
    io: TcpStream,
}

impl Connection {
    pub fn new(addr: SocketAddrV4) -> Self {
        let io = TcpStream::connect(addr).expect("Connect error");
        println!("Connected to the server!");

        Self { io }
    }

    fn send_command(&mut self, bytes: &[u8]) {
        match self.io.write(bytes) {
            Ok(_) => todo!(),
            Err(_) => panic!("send command error"),
        }
    }

    fn read_response(&mut self) {
        let mut buf = [0; 512];
        self.io.read(&mut buf[..]);
    }
}
