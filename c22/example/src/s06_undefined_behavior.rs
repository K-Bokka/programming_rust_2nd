pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("22.6 undefined behavior");

    let i = 10;
    very_trustworthy(&i);
    println!("{}", i * 100);
    Ok(())
}
fn very_trustworthy(_shared: &i32) {
    // unsafe {
    //     let mutable = shared as *const i32 as *mut i32;
    //     *mutable = 20;
    // }
}
