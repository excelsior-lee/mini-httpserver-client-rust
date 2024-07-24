use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9898").unwrap();
    println!("listener on 9898!");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection established!");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    };
}
