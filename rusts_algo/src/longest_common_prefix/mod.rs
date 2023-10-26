
use std::collections::HashMap;

pub fn exec(a:&Vec<&str>) -> String {

    let mut buf = String::new();
    let mut h:HashMap<usize, char> = HashMap::new();
    let mut minlen = std::i32::MAX;
    let mut clen = 0;

    for s in a {
        let bs = s.as_bytes();
        minlen = minlen.min(bs.len() as i32);
        clen = 0;
        for i in 0..minlen as usize {
            if !h.contains_key(&i) {
                h.insert(i, bs[i as usize] as char);
            } else if bs[i] as char == *h.get(&i).unwrap() {
                clen += 1;
            } else {
                clen = clen.min(i as i32);
                minlen = minlen.min(clen);
                break;
            }
        }
    }

    let s:String = a[0].chars().into_iter().collect();
    let t = &s.as_str()[..clen as usize];
    println!("input - {:?}, result - {}", a, t);
    t.to_string()

}

pub fn run() {
    exec(&Vec::from_iter(["flower", "flow", "flight"]));
    exec(&Vec::from_iter(["dog", "racecar", "car"]));
}
