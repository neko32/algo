
pub fn exec(a:&[i32]) -> i32 {
    let mut d = Vec::from_iter(a.clone().iter().cloned());
    let mut sumv = 0;
    for i in 1..d.len() {
        let diff = d[i - 1] - d[i];
        if diff >= 0 {
            d[i] += diff + 1;
            sumv += diff + 1;
        }
    }
    println!("orig - {:?}, derived - {:?}", a, d);
    println!("total - {}", sumv);
    sumv
}

pub fn run() {
    let v = [1, 1, 1];
    exec(&v);
}
