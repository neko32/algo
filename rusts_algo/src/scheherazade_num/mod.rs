
pub fn exec(n:u32) -> Option<u32> {
    let s = n.to_string();
    if s.len() != 3 {
        println!("{} is not valid", n);
        return None;
    }
    let bs = format!("{}{}", s, s);
    let nn:u32 = bs.parse().unwrap();
    let sc = nn / 1001;
    println!("{} -> {:?} scheherazade_num is {}", n, sc, bs);
    Some(sc)
}

pub fn run() {
    exec(682);
    exec(10021);
    exec(50);
    exec(100);
    exec(101);
}
