

pub fn exec(m: u8) -> u8 {
    let q = ((m - 1) / 3) + 1;
    println!("month {m}'s quarter is {q}");
    q
}

pub fn run() {
    for m in 1..=12 {
        exec(m as u8);
    }
}