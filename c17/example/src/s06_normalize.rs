use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn run() {
    println!("17.6 Normalize");

    assert_ne!("th\u{e9}", "the\u{301}");
    assert!("th\u{e9}" > "the\u{301}");

    fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    assert_eq!(hash("th\u{e9}"), 0x53e2d0734eb1dff3);
    assert_eq!(hash("the\u{301}"), 0x90d837f0a0928144);
}
