use std::net::TcpStream;
use std::str;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9898").unwrap();

    // 传输数据
    stream.write("hello world".as_bytes()).unwrap();

    // 读取 response 到 buffer
    let mut buffer = [0; 12];
    stream.read(&mut buffer).unwrap();

    println!("Response : {:?}", str::from_utf8(&buffer).unwrap()); 
}
