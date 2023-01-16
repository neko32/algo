
pub fn exec(v:&[f32]) -> f32 {
    let len = v.len() as f32;
    let s:f32 = v.iter().map(|x|1. / *x).sum();
    len / s
}

pub fn run() {
    let v = [110_f32, 90_f32];
    println!("{}", exec(&v));
}
