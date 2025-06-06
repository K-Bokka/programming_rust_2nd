use std::collections::HashSet;

pub fn run() {
    println!("16.2 Vec<T>");

    let mut numbers: Vec<i32> = vec![];
    numbers.push(1);
    assert_eq!(numbers, vec![1]);

    let words = vec!["hello", "world"];
    assert_eq!(words.join(", "), "hello, world");

    let mut buffer = vec![0u8; 1024];
    buffer[0..4].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(buffer[0..4], [1, 2, 3, 4]);

    let mut my_set = HashSet::new();
    my_set.insert("foo".to_string());
    my_set.insert("bar".to_string());
    let my_vec = my_set.into_iter().collect::<Vec<String>>();
    println!("my_vec: {:?}", my_vec);
}
