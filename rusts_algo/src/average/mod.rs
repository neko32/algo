

pub fn exec(v:&[f32]) -> f32 {
    let len = v.len();
    let sumv:f32 = v.iter().sum();
    let avg = sumv / len as f32;
    println!("{} / {} = {}", sumv, len, avg);
    avg
}

pub fn run() {
    let v = &[80_f32, 50_f32, 30_f32, 20_f32, 70_f32, 80_f32, 100_f32, 40_f32];
    exec(v);
}