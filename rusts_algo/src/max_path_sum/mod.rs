
use std::cmp::max;
use std::collections::VecDeque;
use crate::shared::{TreeNode, add_node_not_balanced, traverse_pre};

pub fn exec(root:TreeNode) -> i32 {
    let root_box = Box::new(root);
    let rez = search(Some(root_box));
    rez.total_max
}

fn search(on:Option<Box<TreeNode>>) -> Info {
    if on.is_none() {
        return Info::new(0, i32::MIN);
    }
    let n = on.unwrap();
    let l = search(n.left);
    let r = search(n.right);
    println!("@{}", n.value);
    let subpath_max = max(l.sub_path_max, r.sub_path_max);
    println!("max sub path? {} - {}", l.sub_path_max, r.sub_path_max);
    let subpath_or_self_max = max(subpath_max + n.value, n.value);
    println!("max sub path or self? {} - {}", subpath_max + n.value, n.value);
    let subpath_or_self_or_path_max = max(subpath_or_self_max, l.sub_path_max + r.sub_path_max + n.value);
    println!("sub path or self or path to this or whole path? {} - {}", subpath_or_self_max, l.sub_path_max + r.sub_path_max + n.value);
    let final_max_with_l = max(subpath_or_self_or_path_max, l.total_max);
    let final_max_with_lr = max(final_max_with_l, r.total_max);
    println!("final - {} - {} - {}", subpath_or_self_or_path_max, l.total_max, r.total_max);
    Info::new(subpath_or_self_max, final_max_with_lr)
}

pub fn run() {
    let mut root = Box::new(TreeNode { value: 1, left: None, right: None});
    let mut op1 = VecDeque::from_iter(["left"]);
    add_node_not_balanced(&mut root, 2, &mut op1);
    let mut op2 = VecDeque::from_iter(["left", "left"]);
    add_node_not_balanced(&mut root, 4, &mut op2);
    let mut op3 = VecDeque::from_iter(["left", "right"]);
    add_node_not_balanced(&mut root, 5, &mut op3);
    let mut op4 = VecDeque::from_iter(["right"]);
    add_node_not_balanced(&mut root, 3, &mut op4);
    let mut op5 = VecDeque::from_iter(["right", "left"]);
    add_node_not_balanced(&mut root, 6, &mut op5);
    let mut op6 = VecDeque::from_iter(["right", "right"]);
    add_node_not_balanced(&mut root, 7, &mut op6);

    let rez = exec(*root);
    println!("{:?}", rez);
}

#[derive(Debug)]
struct Info {
    sub_path_max:i32,
    total_max:i32,
}

impl Info {
    fn new(s:i32, t:i32) -> Self {
        Info {sub_path_max: s, total_max: t}
    }
}