use async_std::io::prelude::*;
use std::io::{Read, Write};
use std::pin::Pin;
use std::rc::Rc;
use std::task::Context;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("20.1 Sync programming to Async programming");

    println!("20.1.1 Future");

    println!("20.1.2 async/await");

    println!("20.1.3 async fn call from sync code");
    // cheapo_request_sync()?;

    println!("20.1.4 async task exec");

    println!("20.1.5 async block");

    let input = async_std::io::stdin();
    #[allow(unused_variables)]
    let future = async {
        let mut line = String::new();
        input.read_line(&mut line).await?;

        println!("You entered: {}", line);

        // Ok(line) // error[E0282]: type annotations needed
        Ok::<(), std::io::Error>(())
    };

    println!("20.1.6 async function using async block");

    println!("20.1.7 async task in thread pool");

    println!("20.1.8 implemented this Future Send?");

    async fn some_async_fn() {
        let _ = cheapo_request_async("example.com", 80, "/");
    }

    #[allow(dead_code)]
    async fn reluctant() -> String {
        let string = Rc::new("ref-counted string".to_string());

        some_async_fn().await;

        format!("You splendid string: {}", string)
    }

    // async_std::task::spawn(reluctant()); // error: future cannot be sent between threads safely

    async fn reluctant_2() -> String {
        let return_value = {
            let string = Rc::new("ref-counted string".to_string());
            format!("You splendid string: {}", string)
        };

        some_async_fn().await;

        return_value
    }
    async_std::task::spawn(reluctant_2());

    type GenericError = Box<dyn std::error::Error>;
    type GenericResult<T> = Result<T, GenericError>;

    fn some_failing_fn() -> GenericResult<i32> {
        Err("error".into())
    }

    async fn use_output(v: i32) {
        println!("value: {}", v);
    }

    #[allow(dead_code)]
    async fn unfortunate() {
        match some_failing_fn() {
            Ok(value) => use_output(value).await,
            Err(error) => println!("error: {}", error),
        }
    }

    // async_std::task::spawn(unfortunate()); // error: future cannot be sent between threads safely

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
async fn cheapo_request_async(host: &str, port: u16, path: &str) -> std::io::Result<String> {
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
    let response = task::block_on(cheapo_request_async("example.com", 80, "/"))?;
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
        handles.push(task::spawn(cheapo_owing_request(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

async fn cheapo_owing_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request_async(&host, port, &path).await
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

#[allow(dead_code)]
async fn many_requests_block(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn(async move {
            cheapo_request_async(&host, port, &path).await
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

#[allow(dead_code)]
fn cheapo_request_async_block<'a>(
    host: &'a str,
    port: u16,
    path: &'a str,
) -> impl std::future::Future<Output = std::io::Result<String>> + 'a {
    async move {
        let mut socket = async_std::net::TcpStream::connect((host, port)).await?;

        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(std::net::Shutdown::Write)?;

        let mut response = String::new();
        socket.read_to_string(&mut response).await?;

        Ok(response)
    }
}

#[allow(dead_code)]
fn cheapo_request_2(
    host: &str,
    port: u16,
    path: &str,
) -> impl std::future::Future<Output = std::io::Result<String>> {
    let host = host.to_string();
    let path = path.to_string();
    async move {
        let mut socket = async_std::net::TcpStream::connect((&*host, port)).await?;

        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(std::net::Shutdown::Write)?;

        let mut response = String::new();
        socket.read_to_string(&mut response).await?;

        Ok(response)
    }
}
