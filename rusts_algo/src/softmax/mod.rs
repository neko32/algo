use rust_decimal::prelude::*;

pub fn exec(x:Vec<f32>) -> Vec<f32> {
    // first napaier to make all postive
    let y1:Vec<f32> = x.iter().map(|v|v.exp()).collect();
    let sumv:f32 = y1.iter().sum();
    let y2:Vec<f32> = y1.into_iter().map(|v|{
        let d = Decimal::from_f32(v / sumv).unwrap();
        d.round_dp(3).to_f32().unwrap()
    }).collect();
    println!("input - {:?}, output - {:?}", x, y2);
    y2
}

pub fn run() {
    exec(vec![1.6, -2.3, 0.2, 3.4, -1.7, 0.5]);
}