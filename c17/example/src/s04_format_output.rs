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
}
