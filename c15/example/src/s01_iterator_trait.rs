pub fn run() {
    println!("15.1 Iterator, IntoIterator trait");

    println!("There's: ");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("1st: {}", element);
    }

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("2nd: {}", element);
    }
}
