use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9796").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf_n = [0u8; 4];
        stream.read_exact(&mut buf_n).unwrap();
        let n = u32::from_be_bytes(buf_n);
        let mut buf = Vec::<u8>::new();
        buf.resize(n as usize, 0);
        let s = stream.read(&mut buf).expect("Cannot read");
        let msg = String::from_utf8_lossy(&buf);
        println!("Recieve message {buf:?} with size {s}");
        println!("Message : {msg}");
    }
}