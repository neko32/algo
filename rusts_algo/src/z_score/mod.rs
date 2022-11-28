
use crate::mean;
use crate::stdev;

pub fn exec(v:&mut [f32]) -> Vec<f32> {
    let mean = mean::exec(v);
    let stdev = stdev::exec(v);
    let zscores:Vec<f32> = v.iter().map(|raw_score|{
        (raw_score - mean) / stdev
    }).collect();
    println!("input - {:?}, mean - {}, stdev - {}, z-score - {:?}", v, mean, stdev, zscores);
    zscores
}

pub fn run() {
    let mut v = [7_f32, 8_f32, 8_f32, 7.5_f32, 9_f32];
    exec(&mut v);
}