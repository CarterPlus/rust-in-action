

/** 在Rust下，函数是一等公民，可以作为参数或者返回值 */
pub fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

pub fn square(value: i32) -> i32 {
    value * value
}

pub fn cube(value: i32) -> i32 {
    value * value * value
}
