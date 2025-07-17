mod s01_reader_writer;
mod s02_file_directory;
mod s03_network_program;

fn main() {
    println!("Chapter 18 input/output");

    println!();
    s01_reader_writer::run();

    println!();
    s02_file_directory::run().unwrap();

    println!();
    s03_network_program::run().unwrap();
}
