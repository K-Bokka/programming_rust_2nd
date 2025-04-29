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

    // 11.4.4
    let zero = 0.0;
    let one = add_one(zero);
    println!("one: {}", one);

    // 11.5
    let doted = dot(&[1, 2, 3, 4], &[1, 1, 1, 1]);
    println!("doted: {}", doted);
    let doted2 = dot2(&[1.0, 2.0, 3.0, 4.0], &[1.0, 1.0, 1.0, 1.0]);
    println!("doted2: {}", doted2);
    let doted3 = dot3(&[1.0, 2.0, 3.0, 4.0], &[1.0, 1.0, 1.0, 1.0]);
    println!("doted3: {}", doted3);

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

use num::Num;
use std::iter;
use std::ops::{Add, Mul};
use std::vec::IntoIter;

#[allow(dead_code)]
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[allow(dead_code)]
fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

#[allow(dead_code)]
fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[allow(dead_code)]
trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
impl Float for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

fn add_one<T: Float + Add<Output = T>>(n: T) -> T {
    n + T::ONE
}

#[allow(dead_code)]
fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib(n - 2),
    }
}

fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn dot2<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot2() {
    assert_eq!(dot2(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot2(&[1.0, 2.0, 3.0, 4.0], &[1.0, 1.0, 1.0, 1.0]), 10.0);
}

fn dot3<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}
