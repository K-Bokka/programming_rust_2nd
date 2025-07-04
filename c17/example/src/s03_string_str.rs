pub fn run() {
    println!("17.3 String & str");

    println!("\n17.3.1 Create String");

    let spacey = "man hat tan";
    let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");

    println!("\n17.3.2 Simple check");

    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[5..], "eeping");
    assert_eq!(&full[2..4], "ok");
    assert_eq!(full[..].len(), 11);
    assert_eq!(full[5..].contains("boo"), false);

    let parenthesized = "Rust (饂)";
    assert_eq!(parenthesized[6..].chars().next(), Some('饂'));

    println!("\n17.3.3 add & insert");
    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    use std::fmt::Write;

    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "letters").unwrap();
    writeln!(letter, "His house is in the village though").unwrap();
    assert_eq!(
        letter,
        "Whose letters these are I think I know\nHis house is in the village though\n"
    );

    let left = "partners".to_string();
    let mut right = "crime".to_string();
    assert_eq!(left + " in " + &right, "partners in crime");

    right += " doesn't pay";
    assert_eq!(right, "crime doesn't pay");

    let string = "test".to_string();
    let parenthetical = "(".to_string() + &string + ")";
    assert_eq!(parenthetical, "(test)");

    println!("\n17.3.4 delete, replace");
    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");

    let mut beverage = "a piña colada".to_string();
    beverage.replace_range(2..7, "kahlua");
    assert_eq!(beverage, "a kahlua colada");

    println!("\n17.3.5 search & iterate naming");

    println!("\n17.3.6 pattern matching");
    let haystack = "One fine day, in the middle of the night";
    assert_eq!(haystack.find(","), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));
    assert_eq!(
        "## Elephants".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
        "Elephants"
    );
    let code = "\t    function noodle() { ";
    assert_eq!(
        code.trim_start_matches([' ', '\t'].as_ref()),
        "function noodle() { "
    );
    assert_eq!(
        code.trim_start_matches(&[' ', '\t'][..]),
        "function noodle() { "
    );

    println!("\n17.3.7 search & replace");
    assert!("2017".starts_with(char::is_numeric));

    let quip = "We also know there are known unknowns.";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));

    assert_eq!(
        "The only thing we have to fear is fear itself.".replace("fear", "hope"),
        "The only thing we have to hope is hope itself."
    );
    assert_eq!(
        "`Borrow` and `BorrowMut`".replace(|ch: char| !ch.is_alphabetic(), ""),
        "BorrowandBorrowMut"
    );
    assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");

    println!("\n17.3.8 iterate for text");
    assert_eq!(
        "élan".char_indices().collect::<Vec<_>>(),
        vec![(0, 'é'), (2, 'l'), (3, 'a'), (4, 'n')]
    );
    assert_eq!(
        "élan".bytes().collect::<Vec<_>>(),
        vec![195, 169, b'l', b'a', b'n']
    );
    assert_eq!(
        "jimb:1000:Jim Brandy:".split(':').collect::<Vec<_>>(),
        vec!["jimb", "1000", "Jim Brandy", ""]
    );
    assert_eq!(
        "jimb:1000:Jim Brandy:"
            .split_terminator(':')
            .collect::<Vec<_>>(),
        vec!["jimb", "1000", "Jim Brandy"]
    );
    let poem = "This  is  just　to say\n\
                     I have eaten\n\
                     the plum\n\
                     again\n";
    assert_eq!(
        poem.split_whitespace().collect::<Vec<_>>(),
        vec![
            "This", "is", "just", "to", "say", "I", "have", "eaten", "the", "plum", "again"
        ]
    );

    println!("\n17.3.9 trimming");
    assert_eq!("\t*.rs  ".trim(), "*.rs");
    assert_eq!("\t*.rs  ".trim_start(), "*.rs  ");
    assert_eq!("\t*.rs  ".trim_end(), "\t*.rs");

    assert_eq!("001990".trim_start_matches('0'), "1990");

    let slice = "banana";
    assert_eq!(slice.strip_suffix("na"), Some("bana"));

    println!("\n17.3.10 convert upper case or lower case");
    assert_eq!("Hello".to_uppercase(), "HELLO");
    assert_eq!("Hello".to_lowercase(), "hello");

    use std::str::FromStr;
    println!("\n17.3.11 parse");
    assert_eq!(usize::from_str("12345"), Ok(12345));
    assert_eq!(f64::from_str("12345.6789"), Ok(12345.6789));
    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(f64::from_str("not a float at all").is_err());
    assert!(bool::from_str("TRUE").is_err());
    assert_eq!(char::from_str("a"), Ok('a'));
    assert!(char::from_str("abc").is_err());

    use std::net::IpAddr;
    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    assert_eq!(
        address,
        IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50])
    );

    println!("\n17.3.12 convert to String");
    assert_eq!(format!("{}, wow", "doge"), "doge, wow");
    assert_eq!(format!("{}", true), "true");
    assert_eq!(
        format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0) / 2.0),
        "(0.500, 0.866)"
    );
    let formatted_addr: String = format!("{}", address);
    assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");
    assert_eq!(address.to_string(), "fe80::3ea9:f4ff:fe34:7a50");

    let addresses = vec![address, IpAddr::from_str("192.168.0.1").unwrap()];
    assert_eq!(
        format!("{:?}", addresses),
        "[fe80::3ea9:f4ff:fe34:7a50, 192.168.0.1]"
    );

    println!("\n17.3.13 borrowing");

    println!("\n17.3.14 access as utf-8");

    println!("\n17.3.15 create from utf-8");
    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]
    );

    println!("\n17.3.16 lazy heap");
    fn get_name() -> String {
        std::env::var("USER").unwrap_or("whoever you are".to_string())
    }
    println!("{}", get_name());

    use std::borrow::Cow;
    fn get_name2() -> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| Cow::Owned(v))
            .unwrap_or(Cow::Borrowed("whoever you are"))
    }

    println!("{}", get_name2());

    fn get_title() -> Option<&'static str> {
        Some("Esq.")
    }
    let mut name = get_name2();
    if let Some(title) = get_title() {
        name.to_mut().push_str(", ");
        name.to_mut().push_str(title);
    }
    println!("Greetings, {}", name);

    fn get_name3() -> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| v.into())
            .unwrap_or(Cow::Borrowed("whoever you are"))
    }

    let mut name = get_name3();

    if let Some(title) = get_title() {
        name += ", ";
        name += title;
    }

    if let Some(title) = get_title() {
        write!(name.to_mut(), "{}", title).unwrap();
    }

    println!("17.3.17 string as general collection");
}
