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

    // 5.2.1
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

    // 5.2.2
    let x = 10;
    let y = 20;
    let mut r = &x;

    if random_bool() {
        r = &y;
    }
    assert!(*r == 20 || *r == 10);

    // 5.2.3
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!((*rrr).x, 1000);

    // 5.2.4
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert_eq!(rry, rrx);

    assert_eq!(ry, rx);
    assert!(!std::ptr::eq(rx, ry));

    // assert_eq!(rx, rrx); // error[E0277]: can't compare `{integer}` with `&{integer}`
    assert_eq!(rx, *rrx);

    // 5.2.6
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    // 5.3.1
    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1);
        }
        // assert_eq!(*r, 1); // error[E0597]: `x` does not live long enough
    }

    // 5.3.2
    static mut STASH: &i32 = &128;
    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
    // error: lifetime may not live long enough
    //    --> c05/example/src/main.rs:110:13
    //     |
    // 108 |     fn f(p: &i32) {
    //     |             - let's call the lifetime of this reference `'1`
    // 109 |         unsafe {
    // 110 |             STASH =p;
    //     |             ^^^^^^^^ assignment requires that `'1` must outlive `'static`
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // 5.3.4
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
    // assert_eq!(*s, 0);
    // error[E0597]: `parabola` does not live long enough
    //    --> c05/example/src/main.rs:128:22
    //     |
    // 127 |         let parabola = [9, 4, 1, 0, 1, 4, 9];
    //     |             -------- binding `parabola` declared here
    // 128 |         s = smallest(&parabola);
    //     |                      ^^^^^^^^^ borrowed value does not live long enough
    // 129 |     }
    //     |     - `parabola` dropped here while still borrowed
    // 130 |     assert_eq!(*s, 0);
    //     |     ----------------- borrow later used here

    // 5.3.5
    struct S<'a> {
        r: &'a i32,
    }
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
    // assert_eq!(*s.r, 10);

    #[allow(dead_code)]
    struct D<'a> {
        s: S<'a>,
    }

    struct S2<'a, 'b> {
        x: &'a i32,
        #[allow(dead_code)]
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S2 { x: &x, y: &y };
            r = s.x;
        }
    }
    println!("r: {}", r);
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

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for i in &v[1..] {
        if *i < *s {
            s = i;
        }
    }
    s
}
