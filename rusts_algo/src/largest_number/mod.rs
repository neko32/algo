
pub fn exec(n:u32) -> u64 {
    let l = 10_u64.pow(n) - 1;
    println!("largest number with {} digits is {}", n, l);
    l
}

pub fn run() {
    for i in 1..=10 {
        exec(i);
    }
}
