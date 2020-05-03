use rust_webserver::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    let pool = match ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(e) => panic!("{}", e)
    };

    for stream in listener.incoming().take(2) {
        let connection_stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(connection_stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    thread::sleep(Duration::from_secs(5));
    println!("Request : {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("hello.html").unwrap();

    println!("{}", contents);

    let response = format!("HTTP/1.1 200 OK\r\n\r\n {}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
