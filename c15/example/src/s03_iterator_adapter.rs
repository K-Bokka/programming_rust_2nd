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
}
