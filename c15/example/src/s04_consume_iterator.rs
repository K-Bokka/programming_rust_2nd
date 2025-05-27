pub fn run() {
    println!("15.4 Consume Iterator");

    // 15.4.1 count, sum, product
    fn triangle(n: u64) -> u64 {
        (1..=n).sum()
    }
    assert_eq!(triangle(10), 55);

    fn factrial(n: u64) -> u64 {
        (1..=n).product()
    }
    assert_eq!(factrial(10), 3628800);
}
