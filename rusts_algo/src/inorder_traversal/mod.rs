
use crate::shared::{build_tree, TreeNode};

pub fn exec(v:&Vec<i32>) -> Vec<i32> {
    let root = build_tree(&v);
    let mut tracker:Vec<i32> = Vec::new();
    trav(root, &mut tracker);
    tracker
}

fn trav(n:Box<TreeNode>, tracker:&mut Vec<i32>) {

    if n.left.is_some() {
        trav(Box::new(*n.left.unwrap()), tracker);
    }
    tracker.push(n.value);
    if n.right.is_some() {
        trav(Box::new(*n.right.unwrap()), tracker);
    }
}

pub fn run() {
    let v = vec![5, 9, 2, 10, 1, 4];
    let r = exec(&v);
    println!("{:?}", r);
}