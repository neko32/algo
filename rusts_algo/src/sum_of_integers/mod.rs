
use std::collections::HashSet;

pub fn exec(n:i32) -> HashSet<Vec<i32>> {
    let mut r:HashSet<Vec<i32>> = HashSet::new();
    let mut t:Vec<i32> = Vec::new();
    scan(2, n, &mut t, &mut r);
    println!("n = {n}, result - {:?}({})", r, r.len());
    r
}

fn scan (a:i32, n:i32, t:&mut Vec<i32>, r:&mut HashSet<Vec<i32>>) {
    let tsum:i32 = t.iter().sum();
    if tsum == n {
        r.insert(t.clone());
    } else if tsum > n {
        // no op
    } else {
        for i in a..=n {
            // skip and move to next
            scan(i + 1, n, t, r);
            // add it and move to next
            t.push(i);
            scan(i + 1, n, t, r);
            // back track
            t.pop();
        }
    }
}

pub fn run() {
    exec(9);
}
