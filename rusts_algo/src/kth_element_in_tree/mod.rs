
use crate::shared::{build_tree, TreeNode, traverse};
pub fn exec(n:Box<TreeNode>, k:usize) -> i32 {
    let mut v:Vec<i32> = Vec::new();
    traverse(n, &mut v);
    let last_idx = v.len() - 1;
    let rez = v.get(last_idx - (k - 1)).unwrap();
    println!("tree - {:?}, {k}th elem is {rez}", v);
    *rez
}

pub fn run() {
    let v = vec![15, 5, 2, 1, 3, 5, 20, 17, 22];
    let n = build_tree(&v);
    exec(n, 3);
}
