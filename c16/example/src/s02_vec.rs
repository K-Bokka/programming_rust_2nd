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

    println!();
    println!("16.2.1 access to element");
    let lines = vec![
        "foo".to_string(),
        "bar".to_string(),
        "baz".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ];
    let first_line = &lines[0];
    assert_eq!(*first_line, "foo");

    let numbers = vec![1, 2, 3, 4, 5];
    let fifth_numbers = &numbers[4];
    assert_eq!(*fifth_numbers, 5);
    let second_line = lines[1].clone();
    assert_eq!(second_line, "bar");
    let my_copy = buffer[4..12].to_vec();
    println!("my_copy: {:?}", my_copy);

    if let Some(item) = numbers.first() {
        println!("item: {:?}", item);
    }

    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);

    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap();
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);

    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(), vec![1, 2, 3, 4, 5, 6]);
}
