pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("22.8 raw pointer");

    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);

    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());

    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first: *const &str = &trucks[0];
    let last: *const &str = &trucks[2];
    assert_eq!(unsafe { last.offset_from(first) }, 2);
    assert_eq!(unsafe { first.offset_from(last) }, -2);

    // &vec![42_u8] as *const String; // error[E0606]: casting `&Vec<u8>` as `*const String` is invalid
    &vec![42_u8] as *const Vec<u8> as *const String;

    println!("22.8.1 How to safely dereference raw pointers");

    println!("22.8.2 Example: RefWithFlag");
    use ref_with_flag::RefWithFlag;
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), true);

    println!("22.8.3 Nullable pointers");

    println!("22.8.4 Type size & alignment");
    assert_eq!(size_of::<i64>(), 8);
    assert_eq!(align_of::<(i32, i32)>(), 4);

    let slice: &[i32] = &[1, 3, 9, 27, 81];
    assert_eq!(size_of_val(slice), 20);

    let text: &str = "alligator";
    assert_eq!(size_of_val(text), 9);

    use std::fmt::Display;
    let unmarkable: &dyn Display = &193_u8;
    let remarkable: &dyn Display = &0.0072973525664;
    assert_eq!(size_of_val(unmarkable), 1);
    assert_eq!(size_of_val(remarkable), 8);

    println!("22.8.5 pointer arithmetic");

    #[allow(unused)]
    fn offset<T>(ptr: *const T, count: isize) -> *const T
    where
        T: Sized,
    {
        let bytes_per_element = std::mem::size_of::<T>() as isize;
        let bytes_offset = count * bytes_per_element;
        (ptr as isize).checked_add(bytes_offset).unwrap() as *const T
    }

    println!("22.8.6 Move to, from memory");

    Ok(())
}

fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        Some(x) => x as *const T,
        None => std::ptr::null(),
    }
}

mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    pub struct RefWithFlag<'a, T: 'a> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>,
    }

    impl<'a, T: 'a> RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<'a, T> {
            assert_eq!(align_of::<T>() % 2, 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData,
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }

        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}
