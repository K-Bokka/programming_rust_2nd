use async_std::io::prelude::*;
use std::io::{Read, Write};
use std::pin::Pin;
use std::task::Context;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("20.1 Sync programming to Async programming");

    println!("20.1.1 Future");

    println!("20.1.2 async/await");

    println!("20.1.3 async fn call from sync code");
    // cheapo_request_sync()?;

    Ok(())
}

#[allow(dead_code)]
fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = std::net::TcpStream::connect((host, port))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

#[allow(dead_code)]
async fn async_cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = async_std::net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

#[allow(dead_code)]
fn cheapo_request_sync() -> std::io::Result<()> {
    use async_std::task;
    let response = task::block_on(async_cheapo_request("example.com", 80, "/"))?;
    println!("{}", response);

    Ok(())
}

#[allow(dead_code)]
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

#[allow(dead_code)]
enum Poll<T> {
    Ready(T),
    Pending,
}
