use ::clap::Parser;
use std::error::Error;
use std::io::{Write, stderr};

fn main() {
    let args = Args::parse();

    if let Err(e) = calculate_tides(&args.planet) {
        print_error(&*e);
        std::process::exit(1);
    }
    // calculate_tides()?; // cannot use the `?` operator in a function that returns `()`
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    planet: String,
}

type TideCalcError = Box<dyn Error + Send + Sync + 'static>;
type TideCalcResult<T> = Result<T, TideCalcError>;

fn calculate_tides(planet: &str) -> TideCalcResult<()> {
    if planet.to_lowercase() == "moon" {
        Ok(())
    } else {
        Err("moon not found".into())
    }
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
