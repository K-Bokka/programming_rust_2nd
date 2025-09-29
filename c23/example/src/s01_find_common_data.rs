use std::os::raw::{c_char, c_int};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("23.1 Find common data representation");

    Ok(())
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum git_error_code {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
}

#[repr(C)]
#[allow(dead_code)]
enum Tag {
    Float = 0,
    Int = 1,
}

#[repr(C)]
union FloatOrInt {
    f: f32,
    i: i32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    union: FloatOrInt,
}

#[allow(dead_code)]
fn is_zero(v: Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value {
                tag: Int,
                union: FloatOrInt { i: 0 },
            } => true,
            Value {
                tag: Float,
                union: FloatOrInt { f: num },
            } => num == 0.0,
            _ => false,
        }
    }
}
