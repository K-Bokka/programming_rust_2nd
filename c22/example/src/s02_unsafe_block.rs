pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("22.2 unsafe block");

    let ascii = vec![0x11];

    unsafe {
        let _ =String::from_utf8_unchecked(ascii);
    }

    Ok(())
}
