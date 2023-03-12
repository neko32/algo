

pub fn exec(f:f32) -> f32 {
    // F = 32 + (C * 9 / 5)
    // F - 32 = ((C * 9 / 5))
    // 5(F - 32) = C * 9
    // 5 * (F - 32) / 9 = C
    let c = ((f - 32.) * 5.) / 9.;
    println!("f - {f}, c - {c}");
    c
}

pub fn run() {
    exec(77.);
    exec(53.6);
    exec(32.);
    exec(26.6);
}
