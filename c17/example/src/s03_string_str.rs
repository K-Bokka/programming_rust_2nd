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
}
