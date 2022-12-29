
use std::collections::HashSet;
pub fn exec(divisors:Vec<i32>, k:i32) -> usize {

    let mut s:HashSet<Vec<bool>> = HashSet::new();
    for i in 1..=k {
        s.insert(calc(&divisors, i));
    }
    let rez = s.len();
    println!("divisors - {:?}, k = {k} => num of clans is {rez}", divisors);
    rez
}

fn calc(d:&Vec<i32>, t:i32) -> Vec<bool> {
    d.iter().map(|x|t % x == 0).collect()
}

pub fn run() {
    let d = vec![2, 3, 4];
    let k = 6;
    exec(d, k);
}