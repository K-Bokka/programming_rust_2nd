use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize};
use std::sync::{Arc, Mutex};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("19.3 Sharing mutable state");

    println!("\n19.3.1 What is exclusive lock?");

    println!("\n19.3.2 Mut<T>");

    println!("\n19.3.3 mutability and Mutex");

    println!("\n19.3.4 Isn't exclusive lock enough");

    println!("\n19.3.5 Dead lock");

    println!("\n19.3.6 Poisoned exclusive lock");

    println!("\n19.3.7 multi consumers channel");

    println!("\n19.3.8 RwLock<T>");

    println!("\n19.3.9 Condvar");

    println!("\n19.3.10 Atomic var");

    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let _worker_cancel_flag = cancel_flag.clone();

    println!("\n19.3.11 Global var");

    static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);

    PACKETS_SERVED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    #[allow(dead_code)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    }

    const fn mono_to_rgba(level: u8) -> Color {
        Color {
            red: level,
            green: level,
            blue: level,
            alpha: 0xFF,
        }
    }
    #[allow(dead_code)]
    const WHITE: Color = mono_to_rgba(0xFF);
    #[allow(dead_code)]
    const BLACK: Color = mono_to_rgba(0x00);

    lazy_static! {
        static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
    }

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
