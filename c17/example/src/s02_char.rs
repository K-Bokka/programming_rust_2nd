pub fn run() {
    println!("17.2 char");

    assert_eq!("カニ".chars().next(), Some('カ'));

    println!("\n17.2.1 char category");

    assert!(32u8.is_ascii_whitespace());
    assert!(b'9'.is_ascii_digit());

    let line_tab = '\u{000b}';
    assert_eq!(line_tab.is_whitespace(), true);
    assert_eq!(line_tab.is_ascii_whitespace(), false);

    println!("\n17.2.2 numeric");
    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15, 16), Some('f'));
    assert!(char::is_digit('f', 16));

    println!("\n17.2.3 upper lower");
    let mut upper = 'ß'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    let ch = 'İ';
    let mut lower = ch.to_lowercase();
    assert_eq!(lower.next(), Some('i'));
    assert_eq!(lower.next(), Some('\u{307}'));
    assert_eq!(lower.next(), None);
}
