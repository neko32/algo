
pub fn exec(n:u32, r:u32) -> u32 {
    let rez = permcalc(n, n - r + 1);
    println!("{}P{} = {}", n, r, rez);
    rez
}

fn permcalc(n:u32, r:u32) -> u32 {
    if n < r {
        1
    } else {
        n * permcalc(n - 1, r)
    }
}

pub fn run() {
    let n = 8;
    let r = 6;
    exec(n, r);
}