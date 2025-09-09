mod s01_what_unsafe;
mod s02_unsafe_block;
mod s03_ascii_mod;

fn main() {
    println!("Chapter 22. unsafe code");

    println!();
    s01_what_unsafe::run().unwrap();

    println!();
    s02_unsafe_block::run().unwrap();

    println!();
    s03_ascii_mod::run().unwrap();
}
