mod s01_iterator_trait;
mod s02_create_iterator;
mod s03_iterator_adapter;

fn main() {
    println!("Chapter 15. Iterator");

    let n = 10;
    println!("triangle: 1 ~ {} : {}", n, triangle(n));
    println!("triangle_fold: 1 ~ {} : {}", n, triangle_fold(n));
    
    println!();
    s01_iterator_trait::run();
    
    println!();
    s02_create_iterator::run();
    
    println!();
    s03_iterator_adapter::run();
}

fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn triangle_fold(n: i32) -> i32 {
    (1..=n).fold(0, |sum, i| sum + i)
}
