pub fn run() {
    println!("15.3 Iterator adapter");

    // 15.3.1 map, filter
    let text = "   ponies   \n giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v, vec!["ponies", "giraffes", "iguanas", "squid"]);

    let text = "   ponies   \n giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, vec!["ponies", "giraffes", "squid"]);

    //  消費しないので何もしない...
    let _ = ["earth", "wind", "fire", "water"]
        .iter()
        .map(|elt| println!("{}", elt));
    // for_each を使う必要がある
    ["earth", "wind", "fire", "water"]
        .iter()
        .for_each(|elt| println!("{}", elt));

    let text = "   ponies   \n giraffes\niguanas   \nsquid".to_string();
    let mut v = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line != "iguanas" {
            v.push(line);
        }
    }
    assert_eq!(v, vec!["ponies", "giraffes", "squid"]);

    // 15.3.2 filter_map, flat_map
    use std::str::FromStr;

    let text = "1\nfrond .25   289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|s| f64::from_str(s).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    let text = "1\nfrond .25   289\n3.1415 estuary\n";
    text.split_whitespace()
        .map(|w| f64::from_str(w))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .for_each(|n| println!("{:4.2}", n.sqrt()));

    use std::collections::HashMap;
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Osaka", "Kyoto"]);
    major_cities.insert("France", vec!["Paris", "Lyon", "Marseille"]);
    major_cities.insert("Germany", vec!["Berlin", "Hamburg", "Stuttgart"]);
    let countries = ["Japan", "Germany"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    // 15.3.3 flatten
    use std::collections::BTreeMap;
    let mut cities = BTreeMap::new();
    cities.insert("Japan", vec!["Tokyo", "Osaka", "Kyoto"]);
    cities.insert("France", vec!["Paris", "Lyon", "Marseille"]);
    cities.insert("Germany", vec!["Berlin", "Hamburg", "Stuttgart"]);
    let all_cities: Vec<_> = cities.values().flatten().cloned().collect();
    assert_eq!(
        all_cities,
        vec![
            "Paris",
            "Lyon",
            "Marseille",
            "Berlin",
            "Hamburg",
            "Stuttgart",
            "Tokyo",
            "Osaka",
            "Kyoto"
        ]
    );

    let v = vec![None, Some("day"), None, Some("one")];
    assert_eq!(
        v.into_iter().flatten().collect::<Vec<_>>(),
        vec!["day", "one"]
    );

    fn to_uppercase(s: &str) -> String {
        s.chars().map(char::to_uppercase).flatten().collect()
    }
    assert_eq!(to_uppercase("hello world"), "HELLO WORLD");

    fn to_uppercase2(s: &str) -> String {
        s.chars().flat_map(char::to_uppercase).collect()
    }
    assert_eq!(to_uppercase2("hello world"), "HELLO WORLD");

    // 15.3.4 take, take_while
    let message = "To: jimb\r\n\
                        From: superego <editor@oreilly.com>\r\n\
                        \r\n\
                        Hello, world!\r\n\
                        ";
    for header in message.lines().take_while(|line| !line.is_empty()) {
        println!("{}", header);
    }

    // 15.3.5 skip, skip_while
    for body in message.lines().skip_while(|line| !line.is_empty()).skip(1) {
        println!("{}", body);
    }
}
