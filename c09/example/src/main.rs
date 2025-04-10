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
    println!("r is {:?}", r)
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
