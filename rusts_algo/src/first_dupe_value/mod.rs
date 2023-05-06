
use std::collections::HashSet;

pub fn exec(v:&[i32]) -> i32 {
    let mut mem:HashSet<i32> = HashSet::new();    
    for n in v {
        if mem.contains(n) {
            return *n;
        }
        mem.insert(*n);
    }
    -1
}

pub fn run() {
    println!("{}", exec(&[2, 1, 5, 2, 3, 3, 4]));
    println!("{}", exec(&[2, 1, 5, 7, 9, 3, 4]));
    println!("{}", exec(&[10, 1, 5, 7, 9, 3, 10]));
}
