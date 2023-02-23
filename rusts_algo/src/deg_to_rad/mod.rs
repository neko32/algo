
pub fn exec(degree:f32) -> f32 {
    let r = degree * (std::f32::consts::PI / 180.);
    println!("degree - {degree}, rad - {r}");
    r
}

pub fn run() {
    exec(90.);
    exec(45.);
    exec(37.);
}
