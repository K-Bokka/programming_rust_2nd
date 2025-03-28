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

    // 5.3.6
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

    // 5.3.7
    #[allow(dead_code)]
    fn sum_r_xy(r: &i32, s2: S2) -> i32 {
        r + s2.x + s2.y
    }

    #[allow(dead_code)]
    fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
        (&point[0], &point[1])
    }

    #[allow(dead_code)]
    struct StringTable {
        elements: Vec<String>,
    }
    impl StringTable {
        #[allow(dead_code)]
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    // 5.4
    let v = vec![4, 8, 19, 27, 34, 10];
    // let r =&v;
    // let aside = v;
    // r[0];
    // error[E0505]: cannot move out of `v` because it is borrowed
    //    --> c05/example/src/main.rs:209:17
    //     |
    // 207 |     let v = vec![4, 8, 19, 27, 34, 10];
    //     |         - binding `v` declared here
    // 208 |     let r =&v;
    //     |            -- borrow of `v` occurs here
    // 209 |     let aside = v;
    //     |                 ^ move out of `v` occurs here
    // 210 |     r[0];
    //     |     - borrow later used here

    {
        let r = &v;
        r[0];
    }
    let _aside = v;

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    // extend(&mut wave, &wave);
    // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
    // error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable
    //    --> c05/example/src/main.rs:237:23
    //     |
    // 237 |     extend(&mut wave, &wave);
    //     |     ------ ---------  ^^^^^ immutable borrow occurs here
    //     |     |      |
    //     |     |      mutable borrow occurs here
    //     |     mutable borrow later used by call

    #[allow(unused_mut)]
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    // x += 10; // `x` is assigned to here but it was already borrowed
    // let m = &mut x; // mutable borrow occurs here
    let m = &x;
    println!("r1: {}, r2: {}, m: {}", r1, r2, m);

    // let mut y = 20;
    // let m1 = &mut y;
    // let m2 = &mut y; // second mutable borrow occurs here
    // let z = y; // use of borrowed `y`
    // println!("m1: {}, m2: {}, z: {}", m1, m2, z);

    #[allow(unused_mut)]
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    // let m1 = &mut r.1; // `r` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    println!("r0: {}", r0);

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    println!("m0: {}, r1: {}", m0, r1);

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut f = new_file(open("test"));
    // clone_from(&mut f, &f);
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

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

struct File {
    descriptor: i32,
}

fn new_file(d: i32) -> File {
    File { descriptor: d }
}

#[allow(dead_code)]
fn clone_from(this: &mut File, rhs: &File) {
    close(this.descriptor);
    this.descriptor = dup(rhs.descriptor);
}

fn open(_path: &str) -> i32 {
    10
}

fn close(_d: i32) {}

fn dup(d: i32) -> i32 {
    d
}
