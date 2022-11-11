
pub fn exec(n:u32, first_num:u32) -> u32 {
    let rez = (first_num + (n / 2)) % n;
    println!("n={},first number={}, opposite loc's num={}", n, first_num, rez);
    rez
}

pub fn run() {
    exec(10, 2);
}
