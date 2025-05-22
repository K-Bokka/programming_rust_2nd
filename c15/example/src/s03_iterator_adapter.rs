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
}
