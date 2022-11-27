
use crate::variance;

pub fn exec(v:&[f32]) -> f32 {
    let var = variance::exec(v);
    let stdev = var.sqrt();
    println!("variance - {}, stdev - {}", var, stdev);
    stdev
}

pub fn run() {
    let v = &[71_f32, 80_f32, 89_f32];
    exec(v);
}