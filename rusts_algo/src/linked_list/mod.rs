
use crate::shared::ListNode;

pub fn exec(v:&[i32]) -> Vec<i32> {
    let mut root = ListNode::new(v[0]);
    for x in &v[1..] {
        root.append(*x);
    }
    let mut buf:Vec<i32> = Vec::new();
    root.trav_from(&mut buf);
    buf
}

pub fn run() {
    println!("{:?}", exec(&[1, 2, 3, 4, 5]));
}

