// For 4.3
fn main() {
    let string1 = "somnabulance".to_string();
    let string2 = string1;

    let num1: i32 = 36;
    let num2 = num1;

    // println!("string1: {}", string1); // error[E0382]: borrow of moved value: `string1`
    println!("string2: {}", string2);
    println!("num1: {}", num1);
    println!("num2: {}", num2);

    let l = Label { number: 3 };
    print(l);
    println!("My number is: {}", l.number);
}
#[derive(Clone, Copy)]
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

// #[derive(Clone, Copy)]
// struct StringLabel {
//     number: String, // error[E0204]: the trait `Copy` cannot be implemented for this type
// }

