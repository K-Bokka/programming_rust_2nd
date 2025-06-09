pub fn run() {
    println!("16.3 VecDeque");

    use std::collections::VecDeque;
    let mut v = VecDeque::from(vec![1, 2, 3, 4]);
    v.push_front(0);
    println!("{:?}", v);
}
