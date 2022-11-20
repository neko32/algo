
use rand::prelude::*;

pub fn exec(v:&mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    let len = v.len() - 1;
    for i in (1..len).rev() {
        let ridx = rng.gen_range(0..=i);
        v.swap(i, ridx);
    }
    println!("{:?}", v);
}

pub fn run() {
    let mut v = vec![1,2,3,4,5,6,7,8,9,10];
    exec(&mut v);
}