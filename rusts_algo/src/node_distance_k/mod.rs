
use crate::shared::{TreeNode, add_node_not_balanced};
use std::collections::VecDeque;

pub fn exec(n:Box<TreeNode>, target:i32, k:i32) -> Vec<i32> {
    let mut buf:Vec<i32> = Vec::new();
    trav(&Some(n), &mut buf, target, k);
    println!("result is {:?}", buf);
    buf
}

fn trav(n_opt:&Option<Box<TreeNode>>, buf:&mut Vec<i32>, target:i32, k:i32) -> i32 {
    if n_opt.is_none() {
        return -1;
    }
    let n = n_opt.as_ref().unwrap();

    if n.value == target {
        add_subtree(Some(n.clone()), buf, 0, k);
        return 1;
    }

    let l = trav(&n.left, buf, target, k);
    let r = trav(&n.right, buf, target, k);

    if l == k || r == k {
        buf.push(n.value);
    }

    if l != -1 {
        add_subtree(n.right.clone(), buf, l + 1, k);
        return l + 1;
    }
    if r != -1 {
        add_subtree(n.left.clone(), buf, r + 1, k);
        return r + 1;
    }
    return -1;
}

fn add_subtree(n_opt:Option<Box<TreeNode>>, buf:&mut Vec<i32>, depth:i32, k:i32) {
    if n_opt.is_some() {
        let n = n_opt.unwrap();
        if depth == k {
            buf.push(n.value);
        }
        add_subtree(n.left, buf, depth + 1, k);
        add_subtree(n.right, buf, depth + 1, k);
    }
}

pub fn run() {
    let mut root = Box::new(TreeNode { value: 1, left: None, right: None});
    let mut op1 = VecDeque::from_iter(["left"]);
    add_node_not_balanced(&mut root, 2, &mut op1);
    let mut op2 = VecDeque::from_iter(["right"]);
    add_node_not_balanced(&mut root, 3, &mut op2);
    let mut op3 = VecDeque::from_iter(["left", "left"]);
    add_node_not_balanced(&mut root, 4, &mut op3);
    let mut op4 = VecDeque::from_iter(["left", "right"]);
    add_node_not_balanced(&mut root, 5, &mut op4);
    let mut op5 = VecDeque::from_iter(["right", "right"]);
    add_node_not_balanced(&mut root, 6, &mut op5);
    let mut op6 = VecDeque::from_iter(["right", "right", "left"]);
    add_node_not_balanced(&mut root, 7, &mut op6);
    let mut op7 = VecDeque::from_iter(["right", "right", "right"]);
    add_node_not_balanced(&mut root, 8, &mut op7);
    exec(root, 3, 2);

}
