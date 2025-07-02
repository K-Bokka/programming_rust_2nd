pub fn run() {
    println!("17.4 formating output");

    println!(
        "{:.3}μs: relocated {} at {:#x} tp {:#x}, {} bytes",
        0.84391, "object", 140737488346304_usize, 6299664_usize, 64
    );

    println!("number of {}: {}", "elephants", 19);
    println!("from {1} to {0}", "the grave", "the cradle");
    println!("v = {:?}", "Nemo");
    println!("{:8.2} km/s", 11.186);
    println!("{:20} {:02x} {:02x}", "abc #42", 105, 42);
    println!("{1:02x} {2:02x} {0}", "abc #42", 105, 42);
    println!(
        "{lsb:02x} {msb:02x} {insn}",
        insn = "abc #42",
        lsb = 105,
        msb = 42
    );
    assert_eq!(format!("{{a, c}} ⊂ {{a, b, c}}"), "{a, c} ⊂ {a, b, c}");

    println!("\n17.4.1 formatting text");
    let bookend = "bookend";
    println!("{}", bookend);
    println!("{:4}", bookend);
    println!("{:.12}", bookend);
    println!("{:12.20}", bookend);
    println!("{:^12}", bookend);
    println!("{:>12.20}", bookend);
    println!("{:=^12}", bookend);
    println!("{:*>12.4}", bookend);

    assert_eq!(format!("{:4}", "th\u{e9}"), "th\u{e9} ");
    assert_eq!(format!("{:4}", "th\u{301}"), "th\u{301} ");

    println!("\n17.4.2 formatting numbers");
    let num = 1234_i32;
    println!("{}", num);
    println!("{:+}", num);
    println!("{:12}", num);
    println!("{:2}", num);
    println!("{:+12}", num);
    println!("{:012}", num);
    println!("{:+012}", num);
    println!("{:<12}", num);
    println!("{:^12}", num);
    println!("{:>12}", num);
    println!("{:<+12}", num);
    println!("{:^+12}", num);
    println!("{:>+12}", num);
    println!("{:=^12}", num);
    println!("{:b}", num);
    println!("{:12o}", num);
    println!("{:+12x}", num);
    println!("{:+12X}", num);
    println!("{:+#12X}", num);
    println!("{:+#012X}", num);
    println!("{:+#06X}", num);

    let float = 1234.5678;
    println!("{}", float);
    println!("{:.2}", float);
    println!("{:.6}", float);
    println!("{:12}", float);
    println!("{:12.2}", float);
    println!("{:12.6}", float);
    println!("{:012.6}", float);
    println!("{:e}", float);
    println!("{:.3e}", float);
    println!("{:12.3e}", float);
    println!("{:12.3E}", float);

    println!("\n17.4.3 other format");

    println!("\n17.4.4 debug format");
    let mut map = std::collections::HashMap::new();
    map.insert("Portland", (45.5237606, -122.6819273));
    map.insert("Taipei", (25.0375167, 121.5637));
    println!("{:?}", map);
    println!("{:#?}", map);

    println!("ordinary: {:02?}", [9, 15, 240]);
    println!("hex: {:02x?}", [9, 15, 240]);

    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone)]
    struct Complex {
        re: f64,
        im: f64,
    }

    let third = Complex {
        re: -0.5,
        im: f64::sqrt(0.75),
    };
    println!("{:?}", third);

    println!("\n17.4.5 pointer format");
    use std::rc::Rc;
    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let imposter = Rc::new("mazurka".to_string());
    println!("text:     {}, {}, {}", original, cloned, imposter);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, imposter);

    println!("\n17.4.6 referencing args by index or name");
    assert_eq!(
        format!("{1}, {0}, {2}", "zeroth", "first", "second"),
        "first, zeroth, second"
    );
    assert_eq!(
        format!("{2:#06x},{1:b},{0:=>10}", "first", 10, 100),
        "0x0064,1010,=====first"
    );
    assert_eq!(
        format!(
            "{description:.<25}{quantity:2} @ {price:5.2}",
            price = 3.25,
            quantity = 3,
            description = "Maple Turmeric Latte"
        ),
        "Maple Turmeric Latte..... 3 @  3.25"
    );
    assert_eq!(
        format!(
            "{mode} {2} {} {}",
            "people",
            "eater",
            "purple",
            mode = "flying"
        ),
        "flying purple people eater"
    );

    println!("\n17.4.7 dynamic field width");

    let content = "hello";
    fn get_width() -> usize {
        10
    }
    fn get_limit() -> usize {
        3
    }

    assert_eq!(format!("{:>20}", content), "               hello");
    assert_eq!(format!("{:>1$}", content, get_width()), "     hello");
    assert_eq!(
        format!("{:>width$}", content, width = get_width()),
        "     hello"
    );
    assert_eq!(
        format!(
            "{:>width$.limit$}",
            content,
            width = get_width(),
            limit = get_limit()
        ),
        "       hel"
    );
    assert_eq!(format!("{:.*}", get_limit(), content), "hel")
}
