use std::io::{self, ErrorKind, Read, Write};

pub fn run() {
    println!("18.1 Reader & Writer");

    #[allow(dead_code)]
    const DEFAULT_BUF_SIZE: usize = 8 * 1024;

    #[allow(dead_code)]
    pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
    where
        R: Read,
        W: Write,
    {
        let mut buf = [0; DEFAULT_BUF_SIZE];
        let mut written = 0;
        loop {
            let len = match reader.read(&mut buf) {
                Ok(0) => return Ok(written),
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            writer.write_all(&buf[..len])?;
            written += len as u64;
        }
    }

    println!("\n18.1.1 Reader");

    println!("\n18.1.2 Reader with buffer");
}
