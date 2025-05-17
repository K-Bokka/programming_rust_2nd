mod s00_intro;
mod s01_capture;
mod s02_f_c_types;
mod s03_c_performance;
mod s04_closure_safety;
mod s05_callback;

fn main() {
    println!("Chapter 14");

    println!();
    s00_intro::run();

    println!();
    s01_capture::run();

    println!();
    s02_f_c_types::run();

    println!();
    s03_c_performance::run();

    println!();
    s04_closure_safety::run();

    println!();
    s05_callback::run();
}
