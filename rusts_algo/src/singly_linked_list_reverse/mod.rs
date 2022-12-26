
use crate::shared::{ListNode, build_singly_linkedlist, Addr};
use crate::shared::Addr::{Nil, Node};

pub fn exec(v:&mut ListNode) -> ListNode {
    let mut ptr = v;
    let mut dummy = ListNode::new(-1);

    loop {
        let mut newn = ListNode::new(ptr.value);
        newn.next = dummy.next;
        dummy.next = Addr::Node(Box::new(newn));
        match ptr.next {
            Nil => break,
            Node(ref mut next) => {
                ptr = next;
            }
        }
    }

    match dummy.next {
        Nil => panic!("shouldn't be nil"),
        Node(next) => *next,
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