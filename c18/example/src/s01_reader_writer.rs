use std::fs::OpenOptions;
use std::io::{self, BufRead, ErrorKind, Read, Write};

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

    println!("\n18.1.3 Read line");

    #[allow(dead_code)]
    fn grep(target: &str) -> io::Result<()> {
        let stdin = io::stdin();
        for line_result in stdin.lock().lines() {
            let line = line_result?;
            if line.contains(target) {
                println!("{}", line);
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn grep_gen<R>(target: &str, reader: R) -> io::Result<()>
    where
        R: BufRead,
    {
        for line_result in reader.lines() {
            let line = line_result?;
            if line.contains(target) {
                println!("{}", line);
            }
        }
        Ok(())
    }

    println!("\n18.1.4 collect for line");

    println!("\n18.1.5 writer");

    println!("\n18.1.6 file");

    #[allow(unused_variables)]
    let log = OpenOptions::new().append(true).open("server.log");
    #[allow(unused_variables)]
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("new_file.txt");

    println!("\n18.1.7 Seek");

    println!("\n18.1.8 Other reader, writer");
    let stdin = io::stdin();
    #[allow(unused_variables)]
    let lines = stdin.lock().lines();
}
