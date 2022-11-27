
pub fn exec(v:&mut [f32]) -> f32 {
    v.sort_by(|a,b|a.partial_cmp(&b).unwrap());
    let len = v.len();
    let mean = match len % 2 {
        0 => (v[(len - 1) / 2] + v[len / 2]) / 2_f32,
        1 => v[len / 2],
        _ => panic!("?"),
    };
    println!("{:?}'s mean = {}", v, mean);
    mean
}

pub fn run() {
    let v = &mut [540_f32, 280_f32, 3000_f32, 540_f32, 480_f32];
    let v2 = &mut [100_f32, 110_f32, 150_f32, 180_f32, 300_f32, 600_f32];
    exec(v);
    exec(v2);
}