
use std::collections::HashMap;
pub fn exec(v:&Vec<&str>) -> Vec<String> {
    let mut h:HashMap<usize, Vec<String>> = HashMap::new();
    let mut max_len:usize = 0;
    for s in v {
        let len = s.len();
        let p = h.entry(len).or_insert(Vec::new());
        p.push(s.to_string());
        max_len = max_len.max(len);
    }
    let rez = h.get(&max_len).unwrap().clone();
    println!("input - {:?}, max_len - {} and result is {:?}", v, max_len, rez);
    rez
}

pub fn run() {
    let v = vec!["aba", "aa", "ad", "vcd", "aba"];
    exec(&v);
}
