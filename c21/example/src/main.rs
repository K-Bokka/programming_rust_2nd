mod s01_macro_basics;
mod s02_built_in_macro;

fn main() {
    println!("Chapter 21 Macro");

    println!();
    s01_macro_basics::run().unwrap();

    println!();
    s02_built_in_macro::run().unwrap();
}
