
pub fn exec(v:&mut [i32]) -> bool {
    let mut cur_idx = 0;
    let mut attempt = 0;
    let max_attempt = v.len();
    let len = v.len();
    while attempt <= max_attempt {
        println!("attempt {attempt}:cur_idx is {cur_idx}");
        if v[cur_idx] == i32::MAX {
            break;
        }
        let t_next_idx = cur_idx as i32 + v[cur_idx];
        v[cur_idx] = i32::MAX;
        if t_next_idx < 0 {
            cur_idx = len - t_next_idx.abs() as usize;
        } else if t_next_idx >= len as i32 {
            cur_idx = (len as i32 - cur_idx as i32).abs() as usize;
        } else {
            cur_idx = t_next_idx as usize;
        }
        attempt += 1;
    }
    println!("at last cur_idx is {cur_idx}");
    attempt == max_attempt && cur_idx == 0
}

pub fn run() {
    println!("{}", exec(&mut [2, 3, 1, -4, -4, 2]));
    println!("{}", exec(&mut [2, 6, 1, -4, -4, 4]));
}
