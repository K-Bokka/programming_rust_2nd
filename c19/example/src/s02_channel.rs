pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("19.2 channel");

    println!("\n19.2.1 send value");

    println!("\n19.2.2 receive value");

    println!("\n19.2.3 exec pipeline");

    println!("\n19.2.4 channel function & performance");

    println!("\n19.2.5 Send & Sync");

    // let rc1 = std::rc::Rc::new("ouch".to_string());
    // let rc2 = rc1.clone();
    // std::thread::spawn(move || { // error[E0277]: `Rc<String>` cannot be sent between threads safely
    //     rc2.clone();
    // });
    // rc1.clone();

    println!("\n19.2.6 connect iterator");

    use std::sync::mpsc;
    #[allow(dead_code)]
    pub trait OffThreadExt: Iterator {
        fn off_thread<T>(self) -> mpsc::IntoIter<Self::Item>;
    }

    use std::thread;
    impl<T> OffThreadExt for T
    where
        T: Iterator + Send + 'static,
        T::Item: Send + 'static,
    {
        fn off_thread<U>(self) -> mpsc::IntoIter<Self::Item> {
            let (sender, receiver) = mpsc::sync_channel(1024);
            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });
            receiver.into_iter()
        }
    }

    println!("\n19.2.7 usage channel without pipeline");

    Ok(())
}
