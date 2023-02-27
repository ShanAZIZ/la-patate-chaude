use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

#[derive(Deserialize)]
pub struct Welcome {
    pub version: String,
}

#[derive(Serialize)]
pub struct Subscribe {
    pub name: String,
}

pub fn hello(stream: &TcpStream) {}

pub fn send_message(stream: &mut TcpStream, message: String) {
    let message = "Hello";
    let buf = message.as_bytes();
    let n = buf.len() as u32;
    let buf_n = n.to_be_bytes();
    stream.write(&buf_n).unwrap();
    stream.write(&buf).unwrap();
}
