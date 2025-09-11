pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("22.3 Example: Efficient ASCII string type");

    use my_ascii::Ascii;

    let bytes: Vec<u8> = b"Hello, world!".to_vec();
    let ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);
    assert_eq!(string, "Hello, world!");

    Ok(())
}

mod my_ascii {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(Vec<u8>);

    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    impl Ascii {
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }

        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            unsafe { String::from_utf8_unchecked(ascii.0) }
        }
    }
}
