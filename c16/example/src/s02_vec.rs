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

    // 16.2.2 iteration

    // 16.2.3 vec extend, shrink
    let mut byte_vec = b"Missssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(byte_vec, b"Misisipi");

    let mut byte_vec = b"Missssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|ch| seen.insert(*ch));
    assert_eq!(byte_vec, b"Misp");

    // 16.2.4 concat, join
    assert_eq!(
        [[1, 2], [3, 4], [5, 6], [7, 8]].concat(),
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    );

    assert_eq!(
        [[1, 2], [3, 4], [5, 6], [7, 8]].join(&0),
        vec![1, 2, 0, 3, 4, 0, 5, 6, 0, 7, 8]
    );

    // 16.2.5 split
    let i = 2;
    let j = 3;

    let v = vec![0, 1, 2, 3];
    let a = &v[i];
    let b = &v[j];
    assert_eq!(a, &2);
    assert_eq!(b, &3);

    let mid = v.len() / 2;
    let front_half = &v[0..mid];
    let back_half = &v[mid..];
    assert_eq!(front_half, [0, 1]);
    assert_eq!(back_half, [2, 3]);

    let mut v = vec![0, 1, 2, 3];
    let a = &mut v[i];
    // let b = &mut v[j]; // error[E0499]: cannot borrow `v` as mutable more than once at a time
    *a = 6;
    // *b = 7;
    assert_eq!(v, vec![0, 1, 6, 3]);

    // 16.2.6 swap
    // 16.2.7 fill
    // 16.2.8 sort
    #[derive(Debug)]
    struct Student {
        first_name: String,
        last_name: String,
        arithmetic: u16,
        science: u16,
        english: u16,
    }

    impl Student {
        fn grade_point_average(&self) -> u16 {
            (&self.arithmetic + &self.science + &self.english) / 3
        }
    }

    let mut students = vec![];
    students.push(Student {
        first_name: "Mike".to_string(),
        last_name: "Smith".to_string(),
        arithmetic: 90,
        science: 80,
        english: 90,
    });
    students.push(Student {
        first_name: "John".to_string(),
        last_name: "William".to_string(),
        arithmetic: 80,
        science: 70,
        english: 80,
    });
    students.sort_by(|a, b| a.first_name.cmp(&b.first_name));
    println!("{:?}", students);

    students.sort_by(|a, b| {
        let a_key = (&a.last_name, &a.first_name);
        let b_key = (&b.last_name, &b.first_name);
        a_key.cmp(&b_key)
    });
    println!("{:?}", students);

    students.sort_by_key(|s| s.grade_point_average());
    println!("{:?}", students);
}
