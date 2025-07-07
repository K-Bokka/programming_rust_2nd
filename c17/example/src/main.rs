mod s01_unicode;
mod s02_char;
mod s03_string_str;
mod s04_format_output;
mod s05_regex;
mod s06_normalize;

fn main() {
    println!("Chapter 17 String, Text");

    println!();
    s01_unicode::run();

    println!();
    s02_char::run();

    println!();
    s03_string_str::run();

    println!();
    s04_format_output::run();

    println!();
    s05_regex::run();

    println!();
    s06_normalize::run();
}
