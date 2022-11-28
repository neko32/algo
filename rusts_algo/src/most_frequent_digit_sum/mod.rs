

use std::collections::HashMap;

pub fn exec(n:i32) -> i32 {
    let mut h:HashMap<i32, i32> = HashMap::new();
    let mut n = n;
    let mut mode = -1;
    let mut mode_key = 0;
    print!("n = {} ", n);
    while n > 0 {
        let ns = n.to_string();
        let bs = ns.as_bytes();
        let d:i32 = bs.iter().map(|b|(*b as char).to_digit(10).unwrap() as i32).sum();
        n -= d;
        let p = h.entry(d).or_insert(0);
        *p += 1;
        let kv = h.get(&d).unwrap();
        if mode == *kv && mode_key < d {
            mode_key = d;
        } else if mode < *kv {
            mode = *kv;
            mode_key = d;
        }
    }
    println!(" result = {} as it appeared {} times", mode_key, mode);
    mode_key
}

pub fn run() {
    let n = 88;
    exec(n);
}