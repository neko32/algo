
use crate::shared::{TreeNode, traverse_pre};
pub fn exec(v:&Vec<i32>) -> Option<Box<TreeNode>> {
    let mut idx:usize = 0;
    reconstruct(&mut idx, &v, std::i32::MIN, std::i32::MAX)
}

fn reconstruct(idx:&mut usize, v:&Vec<i32>, left:i32, right:i32) -> Option<Box<TreeNode>> {
    let len = v.len();
    if *idx == len {
        return None;
    }
    let value = v[*idx];
    if value < left || value >= right {
        return None;
    }
    *idx += 1;

    Some(Box::new(TreeNode {
        value: value,
        left: reconstruct(idx, v, left, value),
        right: reconstruct(idx, v, value, right),
    }))
}

pub fn run() {
    let v:Vec<i32> = vec![10, 4, 2, 1, 5, 17, 19, 18];
    match exec(&v) {
        Some(rez) => {
            let mut trace:Vec<i32> = Vec::new();
            traverse_pre(rez, &mut trace);
            println!("{:?}", trace);
        },
        None => (),
    }
}