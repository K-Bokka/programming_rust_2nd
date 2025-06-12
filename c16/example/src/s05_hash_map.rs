use std::collections::HashMap;

pub fn run() {
    println!("16.5 HashMap & BtreeMap");

    println!("16.5.1 Entry");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Student {
        first_name: String,
        last_name: String,
    }

    let mut student_map = std::collections::HashMap::new();

    let name = "John Doe";
    if !student_map.contains_key(name) {
        student_map.insert(
            name.to_string(),
            Student {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
        );
    }
    let record = student_map.get_mut(name).unwrap();
    println!("{:?}", record);

    let name = "Jane Doe";
    let record = student_map.entry(name.to_string()).or_insert(Student {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
    });
    println!("{:?}", record);

    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    let ballots = vec!["A", "B", "B", "A", "A", "C"];
    for name in ballots {
        let count = vote_counts.entry(name.to_string()).or_insert(0);
        *count += 1;
    }
    println!("{:?}", vote_counts);

    println!("16.5.2 Iteration");

    let mut any_map = HashMap::new();
    any_map.insert("a", 1);
    any_map.insert("b", 2);
    any_map.insert("c", 3);
    for (key, value) in &mut any_map {
        if *key == "a" {
            *value = 10;
        }
    }
    println!("{:?}", any_map);
}
