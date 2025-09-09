mod s01_what_unsafe;
mod s02_unsafe_block;

fn main() {
    println!("Chapter 22. unsafe code");

    println!();
    s01_what_unsafe::run().unwrap();

    println!();
    s02_unsafe_block::run().unwrap();
}
