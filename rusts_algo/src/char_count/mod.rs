
use std::collections::HashMap;

pub fn exec(v:Vec<&str>) -> HashMap<char, u32> {
    let mut c = 'A';
    let guard = '{' as u8;
    let mut h:HashMap<char, u32> = HashMap::new();
    while guard > c as u8 {
        for s in v.iter() {
            let len = s.len() as u32;
            let len_without_c = s.replace(c, "").len() as u32;
            println!("@{c}({}) on {}.. {len} - {len_without_c}", c as u8, s);
            let p = h.entry(c).or_insert(0);
            *p += len - len_without_c;
        }
        c = ((c as u8) + 1) as char
    }
    let rezh:HashMap<char, u32> = h.into_iter().filter(|(_, v)|*v != 0).collect();
    println!("input - {:?}", v);
    println!("result - \n{:?}", rezh);
    rezh
}

pub fn run() {
    exec(vec!["ABC", "ABDabd", "ZzZzZ", "MNN", "D", "ZA", "g"]);
}
