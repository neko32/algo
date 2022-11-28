
use std::cmp::max;
pub fn exec(v:&[i32]) -> i32 {
    let mut largest = i32::MIN;
    let len = v.len();
    if len == 0 {
        panic!("size must be 1 or more");
    } else if len <= 1 {
        return v[0];
    }
    for i in 1..len {
        largest = max(largest, v[i - 1] * v[i]);
    }
    println!("input - {:?}, largest - {}", v, largest);
    largest
}

pub fn run() {
    let v = [3, 6, -2, -5, 7, 3];
    exec(&v);
}
