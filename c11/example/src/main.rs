use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chanter 11 / trait & generics");

    let mut local_file = std::fs::File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"Hello, world!\n");

    let min_i32 = min(1, 2);
    assert_eq!(min_i32, 1);

    // 11.1
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello, world!")?;
    
    Ok(())
}

fn say_hello(out: &mut dyn std::io::Write) -> std::io::Result<()> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}
