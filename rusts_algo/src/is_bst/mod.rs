
use crate::shared::{TreeNode, build_tree} ;

pub fn exec(v:&Vec<i32>) -> bool {
    println!("in - {:?}", v);
    let root = build_tree(v);
    chk(Some(root), i32::MIN, i32::MAX)
}

fn chk(on:Option<Box<TreeNode>>, left:i32, right:i32) -> bool {
    match on {
        None => true,
        Some(n) => {
            let v = n.value;
            if v < left || v >= right {
                false
            } else {
                chk(n.left, left, v) && chk(n.right, v, right)
            }
        }
    }
}

pub fn run() {
    let v = vec![5, 9, 2, 10, 1, 4];
    println!("{}", exec(&v));
}
