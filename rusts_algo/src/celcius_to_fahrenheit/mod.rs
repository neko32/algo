

pub fn exec(c:f32) -> f32 {
    let f = 32. + ((c * 9.) / 5.);
    println!("c - {c}, f - {f}");
    f
}

pub fn run() {
    exec(25.);
    exec(12.);
    exec(0.);
    exec(-3.);
}
