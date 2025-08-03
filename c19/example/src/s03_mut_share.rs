pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("19.3 Sharing mutable state");

    println!("\n19.3.1 What is exclusive lock?");

    println!("\n19.3.2 Mut<T>");

    println!("\n19.3.3 mutability and Mutex");

    println!("\n19.3.4 Isn't exclusive lock enough");

    println!("\n19.3.5 Dead lock");

    println!("\n19.3.6 Poisoned exclusive lock");

    println!("\n19.3.7 multi consumers channel");

    Ok(())
}

#[allow(dead_code)]
pub mod shared_channel {
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};

    pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

    impl<T> Iterator for SharedReceiver<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let guard = self.0.lock().unwrap();
            guard.recv().ok()
        }
    }

    pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
        let (sender, receiver) = std::sync::mpsc::channel();
        (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
    }
}
