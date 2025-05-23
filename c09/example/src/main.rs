use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn main() {
    // 9.1 named field
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let image2 = new_map((1024, 576), vec![0; 1024 * 576]);

    println!(
        "pixel len is {}, image is {:?}",
        image2.pixels.len(),
        image2.size
    );

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    // 9.2 tuple
    let image_bounds = Bounds(1024, 576);
    assert_eq!(image_bounds.0 * image_bounds.1, 1024 * 576);

    let ascii = Ascii(vec![65, 115, 99, 105, 105]);
    assert_eq!(ascii.0, "Ascii".as_bytes().to_vec());

    // 9.3 unit
    #[allow(unused_variables)]
    let o = OneSuch;

    // 9.5 method define
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('Θ');
    assert!(!q.is_empty());
    q.pop();

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split();
    // println!("q is {:?}", q); // value borrowed here after move
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // 9.5.1 self in Box Rc Arc
    let mut bq = Box::new(Queue::new());
    bq.push(' ');
    println!("bq is {:?}", bq);

    // 9.6 Associated constants
    println!(
        "{} id: {}, zero is {:?}",
        Vector2::NAME,
        Vector2::ID,
        Vector2::ZERO
    );
    let scaled = Vector2::UNIT.scaled_by(2.0);
    println!(
        "{} id: {}, scaled is {:?}",
        Vector2::NAME,
        Vector2::ID,
        scaled
    );

    // 9.7 generics structure
    let mut q = QueueG::new();
    let mut r = QueueG::new();
    q.push("CAD");
    r.push(0.74);
    q.push("BTC");
    r.push(13764.0);
    println!("q is {:?}", q);
    println!("r is {:?}", r);

    // 9.8 lifecycle parameter
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.greatest, 48);
    assert_eq!(*e.least, -3);

    // 9.9 generics structure with const
    use std::f64::consts::FRAC_PI_2;
    // sin x =~ x - 1.6x^3 + 1/120x^5
    let sin_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    assert_eq!(sin_poly.eval(0.0), 0.0);
    assert!((sin_poly.eval(FRAC_PI_2) - 1.0).abs() < 0.005);

    // 9.10 generic traits
    let a = Point { x: 1.0, y: 2.0 };
    let b = a.clone(); // Clone
    let c = a; // Copy

    assert_eq!(b, c); // PartialEq
    println!("a is {:?}", a); // Debug

    // 9.11 Interior mutability
    let ref_cell = RefCell::new("hello".to_string());
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }
    let mut w = ref_cell.borrow_mut();
    w.push_str(" world");
    println!("w is {:?}", w);
}
#[derive(Debug)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    #[allow(dead_code)]
    position: (f32, f32, f32),
    #[allow(dead_code)]
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    #[allow(dead_code)]
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

struct Bounds(usize, usize);

struct Ascii(Vec<u8>);

struct OneSuch;
#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    const NAME: &'static str = "Vector2";
    const ID: u32 = 18;

    fn scaled_by(&self, s: f32) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

#[derive(Debug)]
pub struct QueueG<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> QueueG<T> {
    pub fn new() -> Self {
        QueueG {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        } else if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Polynomial { coefficients }
    }
    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = sum * x + self.coefficients[i];
        }
        sum
    }
}

#[allow(dead_code)]
struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N],
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
struct SpiderRoot {
    species: String,
    web_enabled: bool,
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

#[allow(dead_code)]
impl SpiderRoot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
    pub fn log(&self, msg: &str) {
        let mut log_file = self.log_file.borrow_mut();
        writeln!(log_file, "{}", msg).unwrap();
    }
}

#[allow(dead_code)]
pub struct SpiderSenses {
    robot: Rc<SpiderRoot>,
}
