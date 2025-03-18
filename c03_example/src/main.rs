fn main() {
    let v1 = build_vector1();
    let v2 = build_vector2();
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8); // 256 * 3 + 232 = 1000
    assert_eq!(65535_u32 as i16, -1_i16); // 0b_1111_1111_1111_1111

    assert_eq!(-1_i8 as u8, 255_u8); // 0b_1111_1111
    assert_eq!(255_u8 as i8, -1_i8); // 0b_1111_1111

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(-4_i32.abs(), -4);
    assert_eq!(0b110101_u8.count_ones(), 4);

    // println!("{}", -4.abs()); // error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
    println!(" (-4_i32).abs() -> {}", (-4_i32).abs());
    println!("i32::abs(-4) -> {}", i32::abs(-4));
}

fn build_vector1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
