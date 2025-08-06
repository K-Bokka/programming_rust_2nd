use std::io::{Read, Write};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("20.1 Sync programming to Async programming");

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
