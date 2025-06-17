pub fn run() {
    println!("17.2 char");

    assert_eq!("カニ".chars().next(), Some('カ'));
    
    println!("\n17.2.1 char category");
    
    assert!(32u8.is_ascii_whitespace());
    assert!(b'9'.is_ascii_digit());
    
    let line_tab = '\u{000b}';
    assert_eq!(line_tab.is_whitespace(), true);
    assert_eq!(line_tab.is_ascii_whitespace(), false);
    
}
