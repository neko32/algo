
use crate::shared::{traverse_pre, TreeNode};

pub fn exec(v:&[i32]) -> TreeNode {
    let mut sorted_v:Vec<i32> = Vec::new();
    sorted_v.extend_from_slice(v);
    sorted_v.sort();
    *build(&sorted_v, 0, (v.len() - 1) as i32).unwrap()

}

fn build(v:&Vec<i32>, l:i32, r:i32) -> Option<Box<TreeNode>> {
    if l > r {
        None
    } else {
        let m = (l + r) / 2;
        println!("l - {l}, r - {r}, m - {m}");
        Some(Box::new(TreeNode {
            value: v[m as usize],
            left: build(v, l, m - 1),
            right: build(v, m + 1, r)
        }))
    }
}

pub fn run() {
    let v = [6, 3, 2, 1, 5, 4];
    let rez = exec(&v);
    let mut trace:Vec<i32> = Vec::new();
    traverse_pre(Box::new(rez), &mut trace);
    println!("{:?}", trace);
}