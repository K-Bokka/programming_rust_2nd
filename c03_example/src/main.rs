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

    // 真偽値
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // 文字
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);
    assert_eq!('*'.is_alphanumeric(), false);
    assert_eq!('β'.is_alphanumeric(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    // タプル
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // Box
    let t = (12, "eggs");
    let b = Box::new(t);
    assert_eq!(b.0, 12);
    assert_eq!(b.1, "eggs");

    // 配列
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10_000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // ベクタ（可変長配列
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);

    let mut palinddome = vec!["a man", "a plan", "a canal", "panama"];
    palinddome.reverse();
    assert_eq!(palinddome, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);

    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, vec![10, 20, 30, 35, 40, 50]);
    v.remove(1);
    assert_eq!(v, vec![10, 30, 35, 40, 50]);

    let mut v = vec!["Snow Puff", "Grass Gem"];
    assert_eq!(v.pop(), Some("Grass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l, if l.len() % 2 == 0 { "functional" } else { "imperative" });
    }
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

#[allow(dead_code)]
fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}
