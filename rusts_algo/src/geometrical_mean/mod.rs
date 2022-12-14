
pub fn exec(v:Vec<f32>) -> f32 {
    let v:Vec<f32> = v.into_iter().map(|x|(x / 100_f32)).collect();
    println!("percentage - {:?}", v);
    let len = v.len();
    let product:f32 = v.into_iter().product();
    let g_mean = product.powf(1_f32 / len as f32);
    println!("product - {}, g_mean - {}", product, g_mean);
    g_mean
}

pub fn run() {
    let v = vec![125_f32, 160_f32, 200_f32, 150_f32, 125_f32];
    exec(v);
}