
use crate::shared::{build_tree, TreeNode};
use std::collections::VecDeque;
pub fn exec(n:&TreeNode) -> u32 {
    let mut cur_q:VecDeque<&TreeNode> = VecDeque::from_iter([n]);
    let mut cld_q:VecDeque<&TreeNode> = VecDeque::new();
    let mut cnt = 0;
    let mut depth_w = 0;

    while !cur_q.is_empty() {

        while !cur_q.is_empty() {
            let n = cur_q.pop_front().unwrap();
            if n.left.is_some() {
                let cld = n.left.as_ref().unwrap();
                cld_q.push_back(&*cld);
            }
            if n.right.is_some() {
                let cld = n.right.as_ref().unwrap();
                cld_q.push_back(&*cld);
            }
        }
        depth_w += 1;
        cnt += depth_w * cld_q.len();
        println!("depth_w - {depth_w}, total so far - {cnt}");

        while let Some(cld) = cld_q.pop_front() {
            cur_q.push_back(cld);
        }
    }

    cnt as u32
}

pub fn run() {
    let node = build_tree(&vec![50, 20, 100, 10, 30, 80, 150, 5, 15]);
    println!("{}", exec(&node));
}
