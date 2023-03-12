

pub fn exec(line:f32, degree:f32) -> f32 {
    let cos = degree.to_radians().cos();
    let sin = degree.to_radians().sin();
    let y = -(cos / sin) * line + (degree / sin);
    println!("result is {y}");
    y
}

pub fn run() {
    exec(15., 90.);
    exec(15., 45.);
    exec(15., 30.);
}
