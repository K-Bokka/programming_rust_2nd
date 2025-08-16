mod s01_sync_to_async;
mod s02_async_client_server;

fn main() {
    println!("Chapter 20 async programming");

    println!();
    s01_sync_to_async::run().unwrap();

    println!();
    s02_async_client_server::run().unwrap();
}
