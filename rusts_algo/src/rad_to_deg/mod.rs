
pub fn exec(rad:f32) -> f32 {
    // r = d * PI / 180
    // d * PI = r * 180
    // d = r * 180 / PI
    let d = (rad * 180.) / std::f32::consts::PI;
    println!("radian - {rad}, degree - {d}");
    d
}

pub fn run() {
    exec(1.2);
    exec(2.42);
    exec(0.79);
}
