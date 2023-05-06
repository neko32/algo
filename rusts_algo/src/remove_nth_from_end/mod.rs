
use crate::shared::{ListNode, build_singly_linkedlist, Addr};

pub fn exec(mut p:&mut ListNode, nth:usize) {
    let mut cnt = 1;
    let len = p.len();
    let node_idx = len - nth + 1;

    if node_idx == 1 {
        match &mut p.next {
            Addr::Node(ref mut next) => p = next,
            Addr::Nil => (),
        }
        return;
    }

    println!("length - {len}");


    while cnt + 1 != node_idx {
        match &mut p.next {
            Addr::Node(ref mut next) => {
                p = next.as_mut();
                cnt += 1;
            },
            Addr::Nil => panic!("shouldn't be empty"),
        }
    }

    println!("cnt - {cnt}");

    println!("removing {cnt}th node");
    let next_next = match &mut p.next {
        Addr::Node(ref mut next) => {
            next.as_mut()
        },
        Addr::Nil => {
            panic!("something is wrong");
        }
    };
    println!("nextnext - {}", next_next.value);
    p.next = next_next.next.clone();
}


pub fn run() {
    let mut n = *build_singly_linkedlist(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    exec(&mut n, 4);
    let mut v:Vec<i32> = Vec::new();
    n.trav_from(&mut v);
    println!("{:?}", v);
}
