fn main() {
    println!("{}", -100);
    // println!("{}", -100u32); // cannot apply unary operator `-`
    // println!("{}", +100); // unexpected `+`

    let x = 1234.567 % 10.0;
    println!("{}", x); // 4.567000000000007

    let hi: u8 = 0xe0;
    let lo = !hi;
    println!("{}", lo);

    let x = 17;
    let index = x as usize;
    println!("{}", index);

    let f = -1.99;
    let i = f as i32;
    println!("{}", i);

    let x = 1e6;
    let y = x as u8;
    println!("{}", y);

    let is_even = |x: i32| -> bool { x % 2 == 0 };
    println!("{}", is_even(10));
}
