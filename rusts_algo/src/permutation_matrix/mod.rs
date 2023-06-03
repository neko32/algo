
use ndarray::Array2;
use crate::random_perm;

pub fn exec(size:usize) -> Array2<i32> {
    let mut m:Array2<i32> = Array2::zeros([size, size]);
    let mut choices:Vec<i32> = Vec::from_iter(0..size as i32);
    random_perm::exec(&mut choices);

    for i in 0..size {
        let idx = choices[i] as usize;
        m[[i, idx]] = 1;
    }
    println!("input size - {size}, result - ");
    println!("{}", m.view());
    m
}

pub fn run() {
    exec(5);
}
