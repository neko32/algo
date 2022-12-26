
use crate::shared::{ListNode, build_singly_linkedlist, Addr};
use crate::shared::Addr::{Node, Nil};

pub fn exec(root:&mut ListNode) -> ListNode {

    let mut ptr = root;
    let mut dummy = ListNode::new(-1);
    let mut r = &mut dummy;

    loop {
        let newn = ListNode::new(ptr.value);
        r.next = Addr::Node(Box::new(newn));
        match r.next {
            Nil => panic!("..."),
            Node(ref mut next) => {
                r = next;
            }
        }
        match ptr.next {
            Nil => break,
            Node(ref mut next) => {
                ptr = next;
            }
        }
    }

    match dummy.next {
        Node(n) => *n,
        _ => panic!("should be node"),
    }
}

pub fn run() {
    let mut list = build_singly_linkedlist(&vec![1, 2, 3, 4, 5, 6]);
    let mut orig_trace:Vec<i32> = Vec::new();
    list.trav_from(&mut orig_trace);
    let copied = exec(&mut list);
    let mut trace:Vec<i32> = Vec::new();
    copied.trav_from(&mut trace);
    println!("original - {:?}", orig_trace);
    println!("copied - {:?}", trace);
}
