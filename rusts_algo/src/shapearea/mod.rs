
pub fn exec(n:u32) -> u32 {
    let mut sumv = 1;
    for i in 2..=n {
        sumv += arithmetical_prog(i);
    }
    println!("input - {}, result - {}", n, sumv);
    sumv
}

fn arithmetical_prog(n:u32) -> u32 {
    let d = 4;
    0 + (n - 1) * d
}

pub fn run() {
    exec(1);
    exec(2);
    exec(3);
    exec(7000);
}