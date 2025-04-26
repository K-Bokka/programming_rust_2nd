use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chanter 11 / trait & generics");

    let mut local_file = std::fs::File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"Hello, world!\n");

    let min_i32 = min(1, 2);
    assert_eq!(min_i32, 1);

    // 11.1
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello, world!")?;

    let mut buf: Vec<u8> = vec![];
    // let writer: dyn Write = buf; // error[E0277]: the size for values of type `dyn std::io::Write` cannot be known at compilation time
    #[allow(unused_variables)]
    let writer: &mut dyn Write = &mut buf;

    say_hello2(&mut local_file)?;
    say_hello2(&mut bytes)?;

    // let v1 = (0 ..10).collect(); // error[E0283]: type annotations needed
    let v1 = (0..10).collect::<Vec<_>>();
    println!("{:?}", v1);

    // 11.2.1
    let mut out = Sink;
    out.write_all(b"Hello, world!")?;

    // 11.2.2
    assert_eq!('$'.is_emoji(), false);

    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

#[allow(dead_code)]
fn run_query<C: Clone + Debug, D: Copy + Debug>(data: D) -> () {
    println!("data: {:?}", data);
}

#[allow(dead_code)]
fn run_query2<C, D>(data: D) -> ()
where
    C: Clone + Debug,
    D: Copy + Debug,
{
    println!("data: {:?}", data);
}

trait Vegetable {}

// 野菜は1種類に限られる
#[allow(dead_code)]
struct Salad<V: Vegetable> {
    veggies: Vec<V>,
}

// 複数の野菜を格納できる
#[allow(dead_code)]
struct Salad2 {
    veggies: Vec<Box<dyn Vegetable>>,
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

use serde::Serialize;
use serde_json;

pub fn save_config(config: &HashMap<String, String>) -> std::io::Result<()> {
    let writer = File::create("config.json")?;
    let mut serializer = serde_json::Serializer::new(writer);

    config.serialize(&mut serializer)?;
    Ok(())
}
