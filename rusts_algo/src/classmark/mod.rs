
pub fn exec(class_low_v:f32, class_high_v:f32) -> f32 {
    let cm = class_low_v + class_high_v / 2.;
    println!("class_low_v - {class_low_v}, class_high_v - {class_high_v}, class mark - {cm}");
    cm
}

pub fn run() {
    exec(25., 35.);
    exec(0., 10.);
}
