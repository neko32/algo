
pub fn exec(n:i32) -> i32 {
    let ns = n.to_string();
    let cc:Vec<char> = ns.chars().collect();
    let len = ns.len();
    let mut sumv = 0;
    for i in 0..len {
        let idx = len - 1 - i;
        let v = cc.get(idx).unwrap().to_digit(10).unwrap() as i32;
        let mult = 8_i32.pow(i as u32);
        println!("v-{v}, mult-{mult}");
        sumv += v * mult;
    }
    println!("octal - {n}, decimal - {sumv}");
    sumv
}

pub fn run() {
    exec(127);
    exec(5351);
    exec(7);
    exec(8);
    exec(16);
}
