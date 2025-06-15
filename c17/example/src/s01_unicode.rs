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
}
