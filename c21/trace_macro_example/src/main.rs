#![feature(trace_macros)]
fn main() {
    trace_macros!(true);
    let numbers = vec![1, 2, 3];
    trace_macros!(false);
    println!("total: {}", numbers.iter().sum::<i32>());
}

// $ rustup override set nightly
// $ rustc src/main.rs
// note: trace_macro
//  --> src/main.rs:4:19
//   |
// 4 |     let numbers = vec![1, 2, 3];
//   |                   ^^^^^^^^^^^^^
//   |
//   = note: expanding `vec! { 1, 2, 3 }`
//   = note: to `< [_] > :: into_vec($crate :: boxed :: box_new([1, 2, 3]))`
