
pub fn exec(v:Vec<i32>, guard_v:i32) -> i32 {
    let len = v.len();
    let mut sumv:i32 = 0;
    let mut idx = 0;
    while idx < len {
        let val = v[idx];
        if val == guard_v {
            break;
        } else {
            sumv += val;
        }
        idx += 1;
    }
    println!("input vec - {:?}, guard value - {} => sum before guard value {}", v, guard_v, sumv);
    sumv
}

pub fn run() {
    let v = vec![3, 9, 8, 7, 2, 5, 0, 3, 4, 8, 0, 2, 10];
    exec(v, 0);
}