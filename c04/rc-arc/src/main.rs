use std::rc::Rc;

// For 4.4
fn main() {
    let s = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // s.push_str(" noodles"); // error[E0596]: cannot borrow data in an `Rc` as mutable
}
