
use std::collections::{HashSet, HashMap};
pub fn exec(v:&[u64]) -> u32 {
    let mut h:HashMap<u64, HashSet<u64>> = HashMap::new();
    for n in v {
        let key = (n - 1) / 10000;
        let p = h.entry(key).or_insert(HashSet::new());
        p.insert(*n);
    }

    let mut sumv = h.len();
    println!("group num - {}", h.len());
    for (k, s) in h.iter() {
        println!("{k} class num - {}", s.len());
        sumv += s.len();
    }
    println!("input - {:?}, result - {sumv}", v);
    sumv as u32
}

pub fn run() {
    exec(&[20000, 239, 10001, 999999, 10000, 20566, 29999]);
}

