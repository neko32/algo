
use crate::shared::{TreeNode, build_tree};

pub fn exec(n:Box<TreeNode>, target:i32) -> i32 {
    trav(Some(n), target, i32::MAX)
}

fn trav(no:Option<Box<TreeNode>>, target:i32, closest:i32) -> i32 {
    let mut closest = closest;
    match no {
        None => {
            println!("reached to a leaf. ending up with {closest}");
            closest
        }
        Some(n) => {
            let closest_target = (closest - target).abs();
            let n_target = (n.value - target).abs();
            if n_target < closest_target {
                closest = n.value;
            }
            if closest == 0 {
                println!("found exact number with target {target}");
                closest
            } else if target < n.value {
                println!("{target} is less than {}, going down to left", n.value);
                trav(n.left, target, closest)
            } else {
                println!("{target} is bigger than {}, going down to right", n.value);
                trav(n.right, target, closest)
            }
        }
    }
}

pub fn run() {
    let r = build_tree(&vec![10, 5, 2, 1, 15, 13, 14, 22]);
    let rez = exec(r, 20);
    println!("closest value is {rez}");
}
