#![doc = include_str!("../README.md")]
//! Simulate the growth of ferns, from the level of
//! individual cells on up.
//!
//! - link is [`Fern`]
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// Test:
///
/// テストになってしまうのでコメントアウト
/// ```text
///         if samples::everything().works() {
///             println!("Ok")
///         }
///
///     ```
///     if samples::everything().works() {
///         println!("Ok")
///     }
///     ```
/// ```
//
//
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
