
use std::collections::HashMap;
pub fn exec(s:&str) -> &str {
    let mut longest = (0_usize, 1_usize);
    let mut h:HashMap<u8, usize> = HashMap::new();
    let bs = s.as_bytes();
    let mut start_idx = 0;

    for (idx, c) in bs.into_iter().enumerate() {
        if h.contains_key(c) {
            start_idx = (h[c] + 1).max(start_idx);
            if (longest.1 - longest.0) < ((idx + 1) - start_idx) {
                longest = (start_idx, idx + 1);
            }
        }
        h.insert(*c, idx);
    }

    println!("{:?}", h);
    let rez = &s[longest.0..longest.1];
    println!("input - {}, result - {}", s, rez);
    rez
}

pub fn run() {
    exec("akenemvjszazq");
}