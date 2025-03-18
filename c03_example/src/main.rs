fn main() {
    let v1 = build_vector1();
    let v2 = build_vector2();
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

fn build_vector1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
