use std::hash::{BuildHasher, Hash, Hasher};

pub fn run() {
    println!("16.8 Customize hash algorithm");

    #[allow(dead_code)]
    fn compute_hash<B, T>(builder: &B, value: &T) -> u64
    where
        B: BuildHasher,
        T: Hash,
    {
        let mut hasher = builder.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
