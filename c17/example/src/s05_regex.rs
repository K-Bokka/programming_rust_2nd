use regex::Regex;

pub fn run() {
    println!("17.5 Regex");

    println!("\n17.5.1 basic usage");
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.a-zA-Z0-9]*)?").unwrap();

    let haystack = r#"regex = "0.2.5""#;
    assert_eq!(semver.is_match(haystack), true);

    let captures = semver
        .captures(haystack)
        .ok_or("semver regex should have matched")
        .unwrap();
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");

    let haystack = "In the beginning, there was 1.0.0. \
                         For a while, we used 1.0.1-beta, \
                         but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver
        .find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, ["1.0.0", "1.0.1-beta", "1.2.4"]);
}
