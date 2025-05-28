use std::collections::HashMap;

pub fn run() {
    println!("15.4 Consume Iterator");

    // 15.4.1 count, sum, product
    fn triangle(n: u64) -> u64 {
        (1..=n).sum()
    }
    assert_eq!(triangle(10), 55);

    fn factrial(n: u64) -> u64 {
        (1..=n).product()
    }
    assert_eq!(factrial(10), 3628800);

    // 15.4.2 max, min
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    // 15.4.3 max_by, min_by
    use std::cmp::Ordering;
    fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    #[allow(unused_variables)]
    let numbers = [1.0, 4.0, f64::NAN, 2.0];
    // assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panic

    // 15.4.4 max_by_key, min_by_key
    let mut populations = HashMap::new();
    populations.insert("France", 66_000_000);
    populations.insert("Japan", 127_000_000);
    populations.insert("Italy", 60_000_000);
    populations.insert("Germany", 83_000_000);
    populations.insert("UK", 65_000_000);
    populations.insert("US", 323_000_000);
    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"US", &323_000_000))
    );
    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Italy", &60_000_000))
    );
}
