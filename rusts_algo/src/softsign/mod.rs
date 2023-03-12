
pub fn exec(x:f32) -> f32 {
    let r = x / (1. + x.abs());
    println!("{x} -> {r}");
    r
}

pub fn run() {
    exec(7.4);
    exec(0.7);
    exec(0.);
    exec(2.2);
    exec(-0.4);
    exec(-1.9);
}