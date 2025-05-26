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

    // 15.3.6 peekable
    use std::iter::Peekable;

    fn parse_number<I>(tokens: &mut Peekable<I>) -> i32
    where
        I: Iterator<Item = char>,
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                    tokens.next();
                }
                _ => return n as i32,
            }
        }
    }

    let mut chars = "226153890,176639048".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153890);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 176639048);
    assert_eq!(chars.next(), None);

    // 15.3.7 fuse
    struct Flaky(bool);
    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("Totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("Totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("Totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("Totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    // 15.3.8 reversible iterator, rev
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let mut iter = bee_parts.iter().rev();
    assert_eq!(iter.next(), Some(&"abdomen"));
    assert_eq!(iter.next_back(), Some(&"head"));
    assert_eq!(iter.next(), Some(&"thorax"));

    // 15.3.9 inspect
    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after:     {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");

    // 15.3.10 chain
    let v: Vec<i32> = (1..4).chain([20, 30, 40]).collect();
    assert_eq!(v, vec![1, 2, 3, 20, 30, 40]);
    let v: Vec<i32> = (1..4).chain([20, 30, 40]).rev().collect();
    assert_eq!(v, vec![40, 30, 20, 3, 2, 1]);
    
    // 15.3.11 enumerate
    // 2章を参照
    // index 付きのiterator を返す
    // https://github.com/K-Bokka/programming_rust_2nd/blob/9f815df8b9e41c504143f930f36d095997b223ed/c02/mandelbrot/src/main.rs#L30
    
}
