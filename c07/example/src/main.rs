use std::error::Error;
use std::io::{stderr, Write};
use std::io;
use std::fs;
use std::path::Path;

fn main() {
    println!("Chapter 7");
    // pirate_share(1000, 0);
    // attempt to divide by zero
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let hometown = LatLng {
        lat: 37.422,
        lng: -122.084,
    };

    match get_weather(&hometown) {
        Ok(report) => println!("{:?}", &report),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[allow(dead_code)]
fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

#[derive(Debug)]
struct LatLng {
    lat: f64,
    lng: f64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct WeatherReport {
    temperature: f64,
}

fn get_weather(location: &LatLng) -> Result<WeatherReport, String> {
    println!("{:?}", location);
    if location.lat < 0.0 || location.lng < 0.0 {
        Ok(WeatherReport { temperature: 15.0 })
    } else {
        Err("Invalid location".to_string())
    }
}
#[allow(dead_code)]
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}


#[allow(dead_code)]
fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}
