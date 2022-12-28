
use crate::shared::{build_tree, TreeNode};

pub fn exec(v:Vec<i32>) -> Vec<i32> {
    let mut tracker:Vec<i32> = Vec::new();
    let node = build_tree(&v);
    trav(node, &mut tracker);
    tracker
}

fn trav(v:Box<TreeNode>, tracker:&mut Vec<i32>) {
    if v.left.is_some() {
        trav(v.left.unwrap(), tracker);
    }
    if v.right.is_some() {
        trav(v.right.unwrap(), tracker);
    }
    tracker.push(v.value);
}

pub fn run() {
    let v: Vec<i32> = vec![5, 9, 2, 10, 1, 4];
    let r = exec(v);
    println!("{:?}", r);
}