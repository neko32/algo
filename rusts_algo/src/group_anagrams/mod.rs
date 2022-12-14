

use std::collections::HashMap;

pub fn exec<'a>(words:&Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut h:HashMap<String, Vec<&str>> = HashMap::new();

    for word in words {
        let hist = build_hist(word);
        let key = gen_key(hist);
        let p = h.entry(key).or_insert(Vec::new());
        p.push(word);
    }

    let mut r:Vec<Vec<&'a str>> = Vec::new();
    for (_, v) in h {
        r.push(v);
    }
    println!("input words - {:?}", words);
    println!("grouped anagrams - {:?}", r);
    r
}

fn gen_key(h:HashMap<char, u32>) -> String {
    let mut keys:Vec<char> = h.keys().cloned().collect();
    keys.sort();
    keys.into_iter().fold(String::new(), |mut acc, s|{
        let cnt = *h.get(&s).unwrap();
        let v = format!("{}{}", s, cnt);
        acc.push_str(v.as_str());
        acc
    })
}

fn build_hist(w:&str) -> HashMap<char, u32> {
    w.chars().into_iter().fold(HashMap::new(), |mut acc, c|{
        let p = acc.entry(c).or_insert(0);
        *p += 1;
        acc
    })
}

pub fn run() {
    let words = vec!["yo", "act", "flop", "tac", "foo", "cat", "oy", "olfp"];
    exec(&words);
}