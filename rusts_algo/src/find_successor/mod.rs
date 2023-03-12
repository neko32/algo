
use crate::shared::{TreeNode, build_tree};
pub fn exec(root:TreeNode, node_v:i32) -> Option<i32> {
    let mut v:Vec<i32> = Vec::new();
    build_list(Some(Box::new(root)), &mut v);
    let mut idx = 0;
    let len = v.len();
    while idx < len {
        if v[idx] == node_v {
            break;
        }
        idx += 1;
    }
    if idx >= len - 1 {
        println!("{:?} - idx {idx} has value {node_v}. Next node's value is none", v);
        None
    } else {
        let r = v[idx + 1];
        println!("{:?} - idx {idx} has value {node_v}. Next node's value is {r}", v);
        Some(v[idx + 1])
    }
}

fn build_list(no:Option<Box<TreeNode>>, buf:&mut Vec<i32>) {
    if let Some(n) = no {
        build_list(n.left, buf);
        buf.push(n.value);
        build_list(n.right, buf);
    }
}

pub fn run() {
    let n = build_tree(&vec![10, 5, 3, 7, 12, 2]);
    exec(*n, 7);
}
