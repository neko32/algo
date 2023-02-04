
use crate::shared::{TreeNode, build_tree, traverse_pre};
pub fn exec(n:Box<TreeNode>, one:i32, two:i32, three:i32) -> bool {

    let root_cp = n.clone();
    let maybe_node_two = find(Some(n), two);
    match maybe_node_two {
        Some(node_two) => {
            let found_either = check(Some(node_two), &vec![one, three]);
            match found_either {
                Some(node) => {
                    let found_v = node.value;
                    let mut maybe_parent = vec![one, three];
                    if found_v == one {
                        maybe_parent.remove(0);
                    } else {
                        maybe_parent.remove(1);
                    }
                    let parent_node = find(Some(root_cp.clone()), maybe_parent[0]);
                    match parent_node {
                        Some(p_node) => check(Some(p_node), &vec![two]).is_some(),
                        None => false
                    }
                },
                None => {
                    false
                }
            }
        },
        None => {
            false
        }
    }
}

fn find(n:Option<Box<TreeNode>>, target:i32) -> Option<Box<TreeNode>> {
    // println!("{:?} vs {}", n, target);
    match n {
        None => None,
        Some(node) if node.value == target => {
            println!("found - {}", node.value);
            Some(node)
        },
        Some(node) => {
            match find(node.left, target) {
                Some(r) => Some(r),
                None => find(node.right, target),
            }
        }
    }
}

fn check(n:Option<Box<TreeNode>>, target:&Vec<i32>) -> Option<Box<TreeNode>> {
    match n {
        None => None,
        Some(node) => {

            for t in target {
                if node.value == *t {
                    return Some(node)
                }
            }
            let rez = check(node.left, target);
            if rez.is_none() {
                check(node.right, target)
            } else {
                rez
            }
        }
    }
}


pub fn run() {
    let n = build_tree(&vec![5, 2, 1, 0, 4, 3, 7, 6, 8]);
    let one = 5;
    let two = 2;
    let three = 3;
    let mut trace:Vec<i32> = Vec::new();
    traverse_pre(n.clone(), &mut trace);
    println!("tree (pre-trav) - {:?}", trace);
    println!("one - {one}, two - {two}, three - {three}");
    println!("{}", exec(n, one, two, three));
}
