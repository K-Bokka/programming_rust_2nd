mod s01_what_unsafe;
mod s02_unsafe_block;
mod s03_ascii_mod;
mod s04_unsafe_fn;
mod s06_undefined_behavior;
mod s07_unsafe_trait;

fn main() {
    println!("Chapter 22. unsafe code");

    println!();
    s01_what_unsafe::run().unwrap();

    println!();
    s02_unsafe_block::run().unwrap();

    println!();
    s03_ascii_mod::run().unwrap();

    println!();
    s04_unsafe_fn::run().unwrap();

    println!("\n22.5 unsafe block or function");

    println!();
    s06_undefined_behavior::run().unwrap();

    println!();
    s07_unsafe_trait::run().unwrap();
}
