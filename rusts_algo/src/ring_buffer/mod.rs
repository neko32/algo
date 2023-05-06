
pub fn exec(n:i32, rsize:usize) -> Vec<i32> {
    let mut v:Vec<i32> = Vec::from_iter(std::iter::repeat(5).take(rsize));
    for i in 0..n {
        v[i as usize % rsize] = i;
    }
    v.shrink_to_fit();
    println!("n - {n}, rsize - {rsize}, result - {:?}", v);
    v
}

pub fn run() {
    exec(100, 7);
}
