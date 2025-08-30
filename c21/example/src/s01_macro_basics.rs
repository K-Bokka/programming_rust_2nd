macro_rules! my_assert_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`",
                        left_val, right_val
                    )
                }
            }
        }
    }};
}

macro_rules! my_vec {
    ($elem:expr; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ),+ ,) => {
        vec![ $( $x ),* ]
    };
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("21.1 Macro basics");

    my_assert_eq!(1, 1);

    println!("21.1.1 Basics of macros expand to code");

    println!("21.1.2 Unintended consequences");

    println!("21.1.3 reiteration");

    let buffer = my_vec![0_u8; 1000];
    my_assert_eq!(buffer.len(), 1000);

    let numbers = my_vec!["udon", "ramen", "soba"];
    my_assert_eq!(numbers.len(), 3);

    let add_comma = my_vec!["udon", "ramen", "soba",];
    my_assert_eq!(add_comma.len(), 3);

    Ok(())
}
