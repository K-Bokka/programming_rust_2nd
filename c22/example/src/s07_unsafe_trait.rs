pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("22.7 unsafe trait");

    let v: Vec<usize> = zeroed_vector(100_000);
    assert!(v.iter().all(|&x| x == 0));

    Ok(())
}

fn zeroed_vector<T>(len: usize) -> Vec<T>
where
    T: Zeroable,
{
    let mut v = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(v.as_mut_ptr(), 0, len);
        v.set_len(len);
    }
    v
}

unsafe trait Zeroable {}

unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for usize {}
