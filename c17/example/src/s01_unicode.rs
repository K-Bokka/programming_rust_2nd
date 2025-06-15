pub fn run() {
    println!("17.1 Unicode");

    println!("\n17.1.1 ASCII, Latin-1, Unicode");
    fn latin1_to_char(latin1: u8) -> char {
        latin1 as char
    }
    let latin1 = 0xa2;
    let unicode = latin1_to_char(latin1);
    println!("{}", unicode);

    fn char_to_latin1(ch: char) -> Option<u8> {
        if ch as u32 <= 0xff {
            Some(ch as u8)
        } else {
            None
        }
    }
    let unicode = '§';
    let latin1 = char_to_latin1(unicode).unwrap();
    println!("{:02X}", latin1);

    println!("\n17.1.2 UTF-8");
    assert_eq!(
        "うどん: udon".as_bytes(),
        &[
            0xe3, 0x81, 0x86, // う
            0xe3, 0x81, 0xa9, // ど
            0xe3, 0x82, 0x93, // ん
            0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e, // : udon
        ]
    );

    println!("\n17.1.3 Text orientation");
    assert_eq!("שָׁלוֹם".chars().next(), Some('ש'));
}
