
pub fn exec(n:u32, h:u32, r:u32) -> u32 {
    let rez = h * r.pow(n - 1);
    println!("A{} = {} * {}^{} = {}", n, h, r, n - 1, rez);
    rez
}

pub fn run() {
    exec(4, 2, 3);
}