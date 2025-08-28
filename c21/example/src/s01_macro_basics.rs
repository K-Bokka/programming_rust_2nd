macro_rules! my_assert_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`", left_val, right_val)
                }
            }
        }
    });
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("21.1 Macro basics");

    my_assert_eq!(1, 1);

    Ok(())
}
