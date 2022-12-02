

pub fn exec(a:&Vec<i32>) -> Option<(usize, usize)> {
    let mut min_v = i32::MAX;
    let mut max_v = i32::MIN;
    for (idx, v) in a.iter().enumerate() {
        if is_out_of_order(a, idx) {
            min_v = min_v.min(*v);
            max_v = max_v.max(*v);
        }
    }
    if min_v == i32::MAX {
        return None;
    }

    let mut l = 0;
    let mut r = a.len() - 1;
    while a[l] < min_v {
        l += 1;
    }
    while a[r] > max_v {
        r -= 1;
    }
    Some((l, r))
}

fn is_out_of_order(v:&Vec<i32>, idx:usize) -> bool {
    if v.len() < 2 {
        false
    } else {
        match idx {
            0 => v[0] > v[1],
            n if n as usize == v.len() - 1 => v[n as usize - 1] > v[n as usize],
            n => v[n as usize - 1] > v[n as usize] || v[n as usize] > v[n as usize + 1]
        }
    }
}


pub fn run() {
    let a = vec![1, 2, 4, 7, 10, 11, 7, 12, 6, 7, 16, 16, 19];
    println!("input - {:?}", a);
    match exec(&a) {
        Some(rez) => println!("{:?}", rez),
        None => println!("the given array has already sorted"),
    }
}