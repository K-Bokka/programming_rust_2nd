fn main() {
    println!("Chapter 15. Iterator");

    let n = 10;
    println!("triangle: 1 ~ {} : {}", n, triangle(n));
    println!("triangle_fold: 1 ~ {} : {}", n, triangle_fold(n));
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
