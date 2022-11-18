
use std::collections::{HashSet, HashMap};

pub fn exec(s:&String) -> i32 {

    let slen = s.len();
    let s_hash = build_hash(&s);
    let s_set = to_value_set(&s_hash);

    let mut n = 2;
    let mut v:Vec<i32> = Vec::new();
    'lp:loop {
        let sq = n * n;
        let sqlen = sq.to_string().len();
        if sqlen == slen {
            v.push(sq);
        }
        else if sqlen > slen {
            break 'lp;
        }
        n += 1;
    }

    for sq in v.iter().rev() {
        let sq_hash = build_hash(&sq.to_string());
        let sq_set = to_value_set(&sq_hash);
        if sq_set == s_set {
            return *sq;
        }
    }
    -1
}

pub fn run() {
    let s = "aaaabbcde".to_string();
    let rez = exec(&s);
    println!("{} -> {}", s, rez);
}

fn to_value_set(h:&HashMap<char, i32>) -> HashSet<i32> {
    h.values().into_iter().cloned().collect()
}

fn build_hash(s:&String) -> HashMap<char, i32> {
    let mut h:HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let p = h.entry(c).or_insert(0);
        *p += 1;
    }
    h
}