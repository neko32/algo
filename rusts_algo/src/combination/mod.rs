
use crate::permutation;
use crate::factorial;

pub fn exec(n:u32, r:u32) -> u32 {
    let numerator = permutation::exec(n, r);
    let denominator = factorial::exec(r);
    let rez = numerator / denominator;
    println!("{}C{} = {} / {} = {}", n, r, numerator, denominator, rez);
    rez
}

pub fn run() {
    let n = 8;
    let r = 3;
    exec(n, r);
}