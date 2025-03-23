use rand::Rng;

// For chapter 4.2
fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s;
    // let u = s;
    // error[E0382]: use of moved value: `s`
    //  --> c04/os-move/src/main.rs:5:13
    //   |
    // 3 |     let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    //   |         - move occurs because `s` has type `Vec<String>`, which does not implement the `Copy` trait
    // 4 |     let t = s;
    //   |             - value moved here
    // 5 |     let u = s;
    //   |             ^ value used here after move
    //   |
    let t = s.clone();
    let u = s.clone();
    println!("s: {:?}", s);
    println!("t: {:?}", t);
    println!("u: {:?}", u);

    // 4.2.1
    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string();
    println!("s: {:?}", s);
    println!("t: {:?}", t);

    // 4.2.2
    let x = vec![10, 20, 30];
    let c = gen_rnd();

    if (c % 2) == 0 {
        f(x);
    } else {
        g(x);
    };
    // h(x);
    // error[E0382]: use of moved value: `x`
    //   --> c04/os-move/src/main.rs:40:7
    //    |
    // 32 |     let x = vec![10, 20, 30];
    //    |         - move occurs because `x` has type `Vec<u8>`, which does not implement the `Copy` trait
    // ...
    // 36 |         f(x);
    //    |           - value moved here
    // 37 |     } else {
    // 38 |         g(x);
    //    |           - value moved here
    // 39 |     }
    // 40 |     h(x);
    //    |       ^ value used here after move
    //    |

    let mut x = vec![10, 20, 30];
    let mut c = gen_rnd();
    while (c % 2) == 0 {
        g(x);
        x = d();
        c = gen_rnd();
    }
    e(x);

    // 4.2.3
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // let third = v[2]; // error[E0507]: cannot move out of index of `Vec<String>`
    // let fifth = v[4]; // error[E0507]: cannot move out of index of `Vec<String>`

    println!("v: {:?}", v);

    let fifth = v.pop().expect("vectory is empty");
    assert_eq!(fifth, "105");
    println!("Remove fifth v: {:?}", v);

    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    println!("Remove second and swap last: {:?}", v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    println!("Replace third: {:?}", v);

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push_str("!");
        println!("{}", s);
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    // let first_name = composers[0].name; // error[E0507]: cannot move out of index of `Vec<Person>`
    // let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
}

fn f(x: Vec<u8>) {
    println!("f: {:?}", x);
}

fn g(x: Vec<u8>) {
    println!("g: {:?}", x);
}

#[allow(dead_code)]
fn h(x: Vec<u8>) {
    println!("h: {:?}", x);
}

fn gen_rnd() -> i32 {
    let mut rng = rand::rng();
    rng.random()
}

fn d() -> Vec<u8> {
    vec![11, 22, 33]
}

fn e(x: Vec<u8>) {
    println!("e: {:?}", x);
}

struct Person {
    name: Option<String>,
    #[allow(dead_code)]
    birth: i32,
}
