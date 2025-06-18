pub fn run() {
    println!("17.3 String & str");

    println!("\n17.3.1 Create String");
    let spacey = "man hat tan";
    let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");
}
