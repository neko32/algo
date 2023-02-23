
pub fn exec(n:f32) -> u32 {
    let k = n.log2() + 1.;
    let k_rounded = k.round() as u32;
    println!("k = {k}, k_rounded = {k_rounded}, n = {n}");
    k_rounded
}

pub fn run() {
    let n = 9072.;
    exec(n);
}
