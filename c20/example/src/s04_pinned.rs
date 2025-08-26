use async_std::io;
use async_std::io::prelude::*;
use async_std::net;
use std::pin::Pin;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("20.4 Pinned");

    println!("20.4.1 two life stage of future");

    let response = fetch_string("localhost:6502");
    #[allow(unused_variables)]
    let new_variable = response;

    println!("20.4.2 Pinned pointer");

    println!("20.4.3 Unpin trait");
    let mut string = "Pinned?".to_string();
    let mut pinned: Pin<&mut String> = Pin::new(&mut string);

    pinned.push_str(" Not");
    Pin::into_inner(pinned).push_str(" so much.");

    let new_home = string;
    assert_eq!(new_home, "Pinned? Not so much.");

    Ok(())
}

async fn fetch_string(address: &str) -> io::Result<String> {
    let mut socket = net::TcpStream::connect(address).await?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await?;
    Ok(buf)
}
