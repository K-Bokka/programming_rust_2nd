use async_std::io;
use async_std::io::prelude::*;
use async_std::net;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("20.4 Pinned");

    println!("20.4.1 two life stage of future");

    let response = fetch_string("localhost:6502");
    #[allow(unused_variables)]
    let new_variable = response;

    Ok(())
}

async fn fetch_string(address: &str) -> io::Result<String> {
    let mut socket = net::TcpStream::connect(address).await?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await?;
    Ok(buf)
}
