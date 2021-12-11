use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use futures::stream::StreamExt;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            async_std::task::spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, content) = if buffer.starts_with(get) {
        ("200 OK", include_str!("../hello.html"))
    } else if buffer.starts_with(sleep) {
        async_std::task::sleep(Duration::from_secs(5)).await;
        ("200 OK", include_str!("../hello.html"))
    } else {
        ("400 NOT FOUND", include_str!("../404.html"))
    };
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
