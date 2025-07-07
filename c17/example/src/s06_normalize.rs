use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use unicode_normalization::UnicodeNormalization;

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

    println!("\n17.6.1 Normalization Forms");

    println!("\n17.6.2 unicode-normalization crate");
    assert_eq!("Phở".nfd().collect::<String>(), "Pho\u{31b}\u{309}");
    assert_eq!("Phở".nfc().collect::<String>(), "Ph\u{1edf}");
    assert_eq!("① Diﬃculty".nfkc().collect::<String>(), "1 Difficulty");
}
