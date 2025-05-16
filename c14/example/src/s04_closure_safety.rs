pub fn run() -> () {
    println!("Closure safety");

    // 14.4.1 killer
    let my_str = "Hello, world!".to_string();
    let f = || drop(my_str);

    f();
    // f(); // error[E0382]: use of moved value: `f`

    // 14.4.2 FnOnce
    #[allow(dead_code)]
    fn call_twice<F>(closure: F)
    where
        F: Fn(),
    {
        closure();
        closure();
    }

    let my_str = "Hello, world!".to_string();
    #[allow(unused_variables)]
    let f = || drop(my_str); // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
    // call_twice(f);

    // 14.4.3 FnMut
    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! is is now: {}", i);
    };

    fn call_twice_mut<F>(mut closure: F)
    where
        F: FnMut(),
    {
        closure();
        closure();
    }

    call_twice_mut(incr);
    assert_eq!(i, 2);

    // 14.4.4 Copy & Clone
    let y = 10;
    let add_y = |x| x + y;
    let add_y_copy = add_y;
    assert_eq!(add_y(add_y_copy(22)), 42);

    let mut x = 0;
    let add_x = |n| {
        x += n;
        x
    };
    let mut add_x_copy = add_x;
    // assert_eq!(add_x(add_x_copy(1)), 2); // error[E0382]: borrow of moved value: `add_x`
    assert_eq!(add_x_copy(1), 1);
    
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}
