pub fn run() -> () {
    println!("Closure safety");
    
    // 14.4.1 killer
    let my_str = "Hello, world!".to_string();
    let f = || drop(my_str);
    
    f();
    // f(); // error[E0382]: use of moved value: `f`
    
    // 14.4.2 FnOnce
    #[allow(dead_code)]
    fn call_twice<F>(closure: F) where F: Fn() {
        closure();
        closure();
    }

    let my_str = "Hello, world!".to_string();
    #[allow(unused_variables)]
    let f = || drop(my_str); // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
    // call_twice(f);
}