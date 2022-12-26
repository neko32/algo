

pub fn exec(w:u32, h:u32) -> u32 {
    let w = w - 1;
    let h = h - 1;
    let numerator = fact(w + h);
    let denominator = fact(w) * fact(h);
    let rez = numerator / denominator;
    println!("{}/{} = {}", numerator, denominator, rez);
    rez
}

fn fact(n:u32) -> u32 {
    match n {
        1 => 1,
        _ => n * fact(n - 1)
    }
}

pub fn run() {
    let w = 2;
    let h = 3;
    exec(w, h);
}