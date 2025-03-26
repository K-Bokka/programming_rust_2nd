use rand::Rng;
use std::collections::HashMap;

// for Chapter 5
fn main() {
    // 5.1
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

    // 5.2
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    // v.sort();
    (&mut v).sort();
    assert_eq!(v, vec![1968, 1973]);

    let x = 10;
    let y = 20;
    let mut r = &x;

    if random_bool() {
        r = &y;
    }
    assert!(*r == 20 || *r == 10);

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!((*rrr).x, 1000);
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

struct Anime {
    name: &'static str,
    #[allow(dead_code)]
    bechdel_pass: bool,
}

fn random_bool() -> bool {
    let mut rng = rand::rng();
    let r = rng.random::<u8>();
    r % 2 == 0
}

struct Point {
    x: i32,
    #[allow(dead_code)]
    y: i32,
}
