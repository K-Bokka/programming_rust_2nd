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

    // 11.3
    let hello1 = "hello".to_string();
    let hello2 = str::to_string("hello");
    let hello3 = ToString::to_string("hello"); // 修飾メソッド呼び出し
    let hello4 = <str as ToString>::to_string("hello"); // 完全修飾メソッド呼び出し
    println!(
        "hello1: {}, hello2: {}, hello3: {}, hello4: {}",
        hello1, hello2, hello3, hello4
    );

    let zero = 0;
    // let zero_abs = zero.abs(); // error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
    let zero_abs = i64::abs(zero);
    println!("zero_abs: {}", zero_abs);

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

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

pub struct CherryTree {
    height: u32,
}

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
        CherryTree {
            height: self.height + other.height,
        }
    }
}

pub struct Mammoth {
    name: String,
}

impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
        Mammoth {
            name: format!("{} {}", self.name, other.name),
        }
    }
}

// fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) { // error[E0038]: the trait `Spliceable` is not dyn compatible
//     left.splice(right)
// }

pub trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
}

pub trait StringSet {
    fn new() -> Self
    where
        Self: Sized;

    fn from_slice(slice: &[&str]) -> Self
    where
        Self: Sized;

    fn contains(&self, s: &str) -> bool
    where
        Self: Sized;

    fn add(&mut self, s: &str)
    where
        Self: Sized;
}

#[allow(dead_code)]
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for item in iter {
        results.push(item);
    }
    results
}

// fn dump1<I>(iter: I)
// where
//     I: Iterator,
// {
//     for (index, value) in iter.enumerate() {
//         println!("{}: {:?}", index, value); // error[E0277]: `<I as Iterator>::Item` doesn't implement `Debug`
//     }
// }

#[allow(dead_code)]
fn dump2<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

#[allow(dead_code)]
fn dump3<I>(iter: I)
where
    I: Iterator<Item = String>,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

#[allow(dead_code)]
fn dump4(iter: &mut dyn Iterator<Item = String>) {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

#[allow(dead_code)]
trait Pattern {
    type Match;
    fn search(&self, string: &str) -> Option<Self::Match>;
}

#[allow(dead_code)]
impl Pattern for char {
    type Match = usize;

    fn search(&self, _string: &str) -> Option<Self::Match> {
        todo!()
    }
}
