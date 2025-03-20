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

    // let mut i = 1;
    // loop {
    //     i *= 10; // thread 'main' panicked at c03_example/src/main.rs:30:9:
    // }

    // let mut i: i32 = 1;
    // loop {
    //     i = i.checked_mul(10).expect("multiplication overflowed");
    // }

    // チェック付き演算（範囲を超えたらNoneになる）
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128_i8).checked_div(-1), None);

    // ラップ演算（範囲を超えたらもう一周）
    assert_eq!(100_u16.wrapping_mul(200), 20_000);
    assert_eq!(500_u16.wrapping_mul(500), 53_392); // 65_536 * 3 + 53_392 = 250_000
    assert_eq!(500_i16.wrapping_mul(500), -12_144); // 53_392 = 0b1101_0000_1001_0000
    assert_eq!(5_i16.wrapping_shl(17), 10); // (17 - 16) bit shift

    // 飽和演算（上限と下限がある）
    assert_eq!(32767_i16.saturating_add(10), 32767);
    assert_eq!((-32768_i16).saturating_sub(10), -32768);

    // オーバーフロー演算
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    assert_eq!(5_u16.overflowing_shl(17), (10, true));

    // 浮動小数点数
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
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
