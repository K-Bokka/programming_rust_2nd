use std::error::Error;
use std::io;

fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
    let mut res = reqwest::blocking::get(url)?;
    if !res.status().is_success() {
        Err(format!("{}", res.status()))?;
    }
    let stdout = io::stdout();
    io::copy(&mut res, &mut stdout.lock())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: http-get URL");
        return;
    }
    if let Err(e) = http_get_main(&args[1]) {
        eprintln!("Error: {}", e);
    }
}
