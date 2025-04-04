pub mod leaves;
pub mod roots;
pub mod stems;

#[allow(unused_imports)]
pub use self::leaves::Leaf;
#[allow(unused_imports)]
pub use self::roots::Root;

#[allow(dead_code)]
pub struct Fern {
    pub roots: Vec<Root>,
    pub leaves: Vec<Leaf>,
}
