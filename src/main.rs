use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    let pool = ThreadPool::new(16);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.0\r\n";
    let sleep = b"GET /sleep HTTP/1.0\r\n";

    let (status, content) = if buffer.starts_with(get) {
        ("200 OK", include_str!("../hello.html"))
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(1));
        ("200 OK", include_str!("../hello.html"))
    } else {
        ("400 NOT FOUND", include_str!("../404.html"))
    };
    let response = format!(
        "HTTP/1.0 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
