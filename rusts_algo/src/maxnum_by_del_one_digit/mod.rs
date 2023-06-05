
pub fn exec(n:u32) -> u32 {
    let mut max_n:u32 = std::u32::MIN;
    let ns = n.to_string();
    let s = ns.as_str();
    for i in 0..s.len() {
        let a = &s[0..i];
        let b = &s[i + 1 ..];
        let c = format!("{}{}", a, b);
        let cn:u32 = c.parse().unwrap();
        println!("{cn} vs {max_n}");
        max_n = max_n.max(cn);
    }
    println!("input - {n}, result - {max_n}");
    max_n
}

pub fn run() {
    exec(152);
    exec(1001);
}

