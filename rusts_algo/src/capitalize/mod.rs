
pub fn exec(s:&str) -> String {
    print!("[before - {s}]");
    let bs = s.as_bytes();
    let h = (bs[0] as char).to_ascii_uppercase();
    let mut t:Vec<char> = bs[1..].into_iter().cloned().map(|u|(u as char).to_ascii_lowercase()).collect();
    let mut chars:Vec<char> = Vec::new();
    chars.push(h);
    chars.append(&mut t);
    let rez = String::from_iter(&chars[0..]);
    println!("[after - {rez}]");
    rez
}

pub fn run() {
    exec("ho ho ho");
    exec("HO HO HO");
    exec("ho hO Ho");
}
