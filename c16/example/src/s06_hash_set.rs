use std::collections::HashSet;

pub fn run() {
    println!("16.6 HashSet & BtreeSet");

    println!("16.6.1 set iter");
    let mut hs = HashSet::new();
    hs.insert("hoge");
    hs.insert("fuga");
    hs.insert("piyo");

    for h in &hs {
        println!("{}", h);
    }
    // for h in &mut hs { // error[E0277]: `&mut HashSet<&str>` is not an iterator
    //     println!("{}", h);
    // }

    println!("16.6.2 equal value. but not");
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    println!("{:p}", &s1 as &str);
    println!("{:p}", &s2 as &str);

    println!("16.6.3 calculate set");
    let hs1 = HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f']);
    let hs2: HashSet<_> = HashSet::from_iter(vec!['a', 'c', 'd', 'f', 'g']);
    println!("{:?}", &hs1 & &hs2);
    
}
