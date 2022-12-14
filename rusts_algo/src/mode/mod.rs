
use std::collections::HashMap;

pub fn exec(v:&Vec<i32>) -> i32 {
    let h:HashMap<i32, i32> = v.iter().fold(HashMap::new(), |mut acc, n|{
        let p = acc.entry(*n).or_insert(0);
        *p += 1;
        acc
    });
    let n = h.into_iter().max_by_key(|(_, v)|*v)
    .map(|(k, _)|k).unwrap();
    println!("input - {:?}, mode - {}", &v, n);
    n
}

pub fn run() {
    let v = vec![3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
    exec(&v);
}