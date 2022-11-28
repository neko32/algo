
use std::collections::HashMap;
pub fn exec(s:&str) -> String {
    let mut h:HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        let p = h.entry(c).or_insert(0);
        *p += 1;
    }
    println!("{:?}", h);
    let mut d:HashMap<char, u32> = HashMap::new();
    d.insert('0', *h.get(&'z').unwrap_or(&0));
    d.insert('2', *h.get(&'w').unwrap_or(&0));
    d.insert('4', *h.get(&'u').unwrap_or(&0));
    d.insert('6', *h.get(&'x').unwrap_or(&0));
    d.insert('8', *h.get(&'g').unwrap_or(&0));
    d.insert('3', *h.get(&'h').unwrap_or(&0) - d[&'8']);
    d.insert('5', *h.get(&'f').unwrap_or(&0) - d[&'4']);
    d.insert('7', *h.get(&'s').unwrap_or(&0) - d[&'6']);
    d.insert('9', *h.get(&'i').unwrap_or(&0) - d[&'5'] - d[&'6'] - d[&'8']);
    d.insert('1', *h.get(&'n').unwrap_or(&0) - d[&'7'] - d[&'9'] * 2);
    let mut digits:Vec<(_, _)> = d.iter().collect();
    digits.sort_by(|a, b|{a.0.cmp(b.0)});

    let mut buf:String = String::new();
    for (d, n) in digits {
        for _ in 0..*n as usize {
            buf.push(*d);
        }
    }
    println!("input - {}, output - {}", s, &buf);
    buf

}

pub fn run() {
    exec("owoztneoer");
    exec("fviefuro");
}