fn main() {
    println!("Hello, world!");
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    1 / 0;
}

#[test]
fn explicit_radix() -> Result<(), std::num::ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.000001
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert_eq!(roughly_equal(PI.sin(), 0.0), true);
    }
}
