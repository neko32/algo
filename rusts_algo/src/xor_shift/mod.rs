
pub fn exec(n:u128) -> u128 {
    let sysdate = std::time::SystemTime::now();
    let mut seed:u128 = sysdate.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    println!("original seed value - {}", seed);
    seed ^= seed << 13;
    seed ^= seed >> 7;
    seed ^= seed << 17;
    let rez = seed % n;
    println!("generated value - {}, with the specified range {}", rez, n);
    rez
}

pub fn run() {
    let v:Vec<u128> = Vec::from_iter(std::iter::repeat_with(||exec(10000000)).take(10));
    println!("{:?}", v);
}