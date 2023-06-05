
pub fn exec(v:&[f32]) -> f32 {
    let l1n:f32 = v.into_iter().map(|x|x.abs()).sum();
    println!("input - {:?}, l1 norm - {l1n}", v);
    l1n
}

pub fn run() {
    let v = [1.2, 0.8, -0.5, 0., 2., -1.5];
    exec(&v);
}

