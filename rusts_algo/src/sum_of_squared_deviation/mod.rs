
pub fn exec(v:&Vec<f32>) -> f32 {
    let avg:f32 = v.iter().cloned().sum::<f32>() / v.len() as f32;
    let s = v.iter().fold(0_f32, |acc, x| {
        let dev = (*x - avg).powf(2.0);
        println!("dev - {}, squared - {}", *x, dev);
        acc + dev
    });
    println!("v - {:?}, len - {}, avg of v - {}, rez - {}", v, v.len(), avg, s);
    s
}

pub fn run() {
    let v:Vec<f32> = vec![4.0, 5.0, 6.0, 7.0, 9.0, 12.0, 15.0, 16.0];
    exec(&v);
}
