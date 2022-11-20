
use std::collections::HashMap;

pub fn exec(a:&Vec<u32>) -> u32 {
    let mut h:HashMap<u32,u32> = HashMap::new();
    for n in a {
        let idx = (n - 1) / 10000;
        let p = h.entry(idx).or_insert(0);
        *p += 1;
    }
    let keynum:u32 = h.keys().len() as u32;
    let valsum:u32 = h.values().sum();
    let rez = keynum + valsum;
    println!("{} + {} = {}", keynum, valsum, rez);
    rez
}

pub fn run() {
    let a = vec![10000, 20000, 30000, 40000, 50000, 60000, 10000, 120000, 150000, 200000, 300000, 1000000, 10000000, 100000000, 10000000];
    exec(&a);
}