
use crate::average;

pub fn exec(v:&[f32]) -> f32 {
    let avg = average::exec(v);
    let dist_sum:f32 = v.iter().map(|x|(x - avg).powf(2_f32)).sum();
    let len = v.len();
    let var = dist_sum / len as f32;
    println!("{} = ({} / {}), average is {}", var, dist_sum, len, avg);
    var
}

pub fn run() {
    let v = &[71_f32, 80_f32, 89_f32];
    exec(v);
}