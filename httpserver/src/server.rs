use super::router::Router;
use http::request::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Server<'a> {
        Server {
            socket_addr,
        }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("connection established!");

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            // 把字节流转化为 HTTP 请求,  to_vec 将字节流转换为 Vec<u8>, String::from_utf8 尝试从 Vec<u8> 读取 String
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);

        }
    }
}