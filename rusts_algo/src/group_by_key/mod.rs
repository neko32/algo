
pub fn exec(l:Vec<(i32, i32)>, group_key:i32) -> Vec<i32> {
    let vals:Vec<_> = l.iter().cloned().filter_map(|(k, v)|{
        if k == group_key { Some(v) } else { None }
    }).collect();
    println!("input - {:?}, values grouped by {group_key} - {:?}", l, vals);
    vals
}

pub fn run() {
    let l = vec![(1, 2), (1, 3), (3, 2), (4, 2), (4,3)];
    exec(l, 1);
}