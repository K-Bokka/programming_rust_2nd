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

    println!("20.1.4 async task exec");

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
#[allow(dead_code)]
async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(cheapo_owing_request(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

async fn cheapo_owing_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    async_cheapo_request(&host, port, &path).await
}

#[allow(dead_code)]
fn cheapo_request_many() {
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("www.rust-lang.org".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_requests(requests));

    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(e) => println!("Error: {}", e),
        }
    }
}
