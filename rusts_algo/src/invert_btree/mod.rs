
use crate::shared::{TreeNode, build_tree, traverse};
pub fn exec(n:&mut Box<TreeNode>) {
    let tl = n.left.as_ref().cloned();
    let tr = n.right.as_ref().cloned();
    n.left = tr;
    n.right = tl;
    if n.left.is_some() {
        exec(&mut n.left.as_mut().unwrap());
    }
    if n.right.is_some() {
        exec(&mut n.right.as_mut().unwrap());
    }
}

pub fn run() {
    let mut n = build_tree(&vec![100, 50, 10, 60, 200, 170, 300, 250, 500]);
    exec(&mut n);
    let mut v:Vec<i32> = Vec::new();
    traverse(n, &mut v);
    println!("{:?}", v);
}
