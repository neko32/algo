
pub fn exec(n:u32) -> u32 {
    let llsd = n / 10_u32.pow(0) % 10;
    println!("n={n}, LSD={llsd}");
    llsd
}

pub fn run() {
    exec(872);
    exec(1111);
    exec(1234);
    exec(5);
    exec(2208893445);
}

