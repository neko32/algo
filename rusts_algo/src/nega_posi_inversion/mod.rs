

pub fn exec(v:u8) -> u8 {
    let inv = 255 - v;
    println!("{v} <-> {inv}");
    inv
}

pub fn run() {
    exec(72);
    exec(0);
    exec(255);
    exec(128);
}
