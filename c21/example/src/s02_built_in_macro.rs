pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("21.2 Built-in Macro");

    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(version, "0.1.0");

    Ok(())
}
