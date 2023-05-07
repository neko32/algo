use std::ops::Neg;

pub fn exec(delta:f32, y:Vec<f32>, t:Vec<f32>) -> f32 {
    println!("delta:{delta}, y:{:?}, t:{:?}", y, t);
    let with_d:Vec<f32> = y.into_iter().map(|v|v + delta).collect();
    let logged:Vec<f32> = with_d.into_iter().map(|v|v.log10()).collect();
    let sum:f32 = logged.into_iter().sum::<f32>().neg();
    println!("result is {sum}");
    sum
}

pub fn run() {
    let d = 1e-7;
    let y = vec![0.7, 0.1, 0.2];
    let t = vec![1., 0., 0.];
    exec(d, y, t);
}
