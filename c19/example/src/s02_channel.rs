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

    Ok(())
}
