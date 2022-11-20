
pub fn exec(n:u32) -> u32 {
    if n == 1 {
        1
    } else {
        n * exec(n - 1)
    }
}

pub fn run() {
    let n = 10;
    let rez = exec(n);
    println!("{}! = {}", n, rez);
}