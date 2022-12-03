
use crate::shared::build_singly_linkedlist;
pub fn exec(v:&Vec<i32>) -> usize {
    let root = build_singly_linkedlist(&v);
    let siz = root.len();
    println!("input - {:?}, size - {}", &v, siz);
    siz
}

pub fn run() {
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    exec(&v);
}