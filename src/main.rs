use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

    for stream in listener.incoming() {
        let stream1 = stream.unwrap();

        handle_connection(stream1);


    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request : {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}