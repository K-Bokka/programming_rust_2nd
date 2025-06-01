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

    // 15.4.5 compare item list
    let packed = "Helen of Troy";
    let spaced = "Helen     of      Troy";
    let obscure = "Helen of Sandusky";
    assert_ne!(packed, spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    // ' ' < 'o'
    assert!(spaced < obscure);
    // 'Troy' > 'Sandusky'
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));

    // 15.4.6 any, all
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    // 15.4.7 position, rposition, ExactSizeIterator
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);
    // assert_eq!(text.chars().rposition(|c| c == 'e'), Some(4)); // error[E0277]: the trait bound `Chars<'_>: ExactSizeIterator` is not satisfied

    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&b| b == b'e'), Some(4));
    assert_eq!(bytes.iter().position(|&b| b == b'X'), Some(0));

    // 15.4.8 fold, rfold
    let numbers = [5, 6, 7, 8, 9, 10];
    assert_eq!(numbers.iter().fold(0, |acc, _| acc + 1), 6);
    assert_eq!(numbers.iter().fold(0, |acc, i| acc + i), 45);
    assert_eq!(numbers.iter().fold(1, |acc, i| acc * i), 151_200);
    assert_eq!(numbers.iter().cloned().fold(i32::MIN, std::cmp::max), 10);

    let words = ["Hello", "World", "Rust"];
    let sentence = words
        .iter()
        .fold(String::new(), |acc, word| acc + word + " ");
    assert_eq!(sentence, "Hello World Rust ");
    let rev_sentence = words
        .iter()
        .rfold(String::new(), |acc, word| acc + word + " ");
    assert_eq!(rev_sentence, "Rust World Hello ");

    // 15.4.9 try_fold, try_rfold
    // see: try_fold.rs

    // 15.4.10 nth, nth_back
    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.nth(3), Some(9));
    assert_eq!(squares.nth(0), Some(16));
    assert_eq!(squares.nth_back(0), Some(81));
    assert_eq!(squares.nth(4), None);

    // 15.4.11 last
    let squares = (0..10).map(|i| i * i);
    assert_eq!(squares.last(), Some(81));
    // println!("{:?}", squares); // error[E0382]: borrow of moved value: `squares`

    // 15.4.12 find, rfind, find_map
    println!("{:?}", populations); // HashMap は並び順を保持していない
    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 400_000_000),
        None
    );
    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 300_000_000),
        Some((&"US", &323000000))
    );
}
