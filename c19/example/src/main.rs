mod s01_fork_join;
mod s02_channel;
mod s03_mut_share;

fn main() {
    println!("Chapter 19 Parallelism");

    println!();
    s01_fork_join::run().unwrap();

    println!();
    s02_channel::run().unwrap();

    println!();
    s03_mut_share::run().unwrap();
}
