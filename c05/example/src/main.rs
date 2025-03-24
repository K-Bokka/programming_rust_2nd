use std::collections::HashMap;

// for Chapter 5
fn main() {
    let mut table = Table::new();
    table.insert(
        "Alice".to_string(),
        vec![
            "The Wonderful Wizard of Oz".to_string(),
            "Harry Potter".to_string(),
        ],
    );
    table.insert("Bob".to_string(), vec!["The Hobbit".to_string()]);
    // show(table);
    // assert_eq!(table["Alice"][0], "The Wonderful Wizard of Oz");
    //error[E0382]: borrow of moved value: `table`
    //   --> c05/example/src/main.rs:9:16
    //    |
    // 5  |     let mut table = Table::new();
    //    |         --------- move occurs because `table` has type `HashMap<String, Vec<String>>`, which does not implement the `Copy` trait
    // ...
    // 8  |     show(table);
    //    |          ----- value moved here
    // 9  |     assert_eq!(table["Alice"][0], "The Wonderful Wizard of Oz");
    //    |                ^^^^^ value borrowed here after move
    //    |
    ref_show(&table);
    assert_eq!(table["Alice"][0], "The Wonderful Wizard of Oz");
    sort_works(&mut table);
    assert_eq!(table["Alice"][0], "Harry Potter");
}

type Table = HashMap<String, Vec<String>>;

#[allow(dead_code)]
fn show(table: Table) {
    for (artist, work) in table {
        println!("works by {}:", artist);
        for work in work {
            println!("  {}", work);
        }
    }
}

fn ref_show(table: &Table) {
    for (artist, work) in table {
        println!("works by {}:", artist);
        for work in work {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
