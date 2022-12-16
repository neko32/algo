
pub fn exec(a:&Vec<i32>, b:&Vec<i32>) -> bool {
    chk(a, b, 0, 0, i32::MIN, i32::MAX)
}

fn chk(a:&Vec<i32>, b:&Vec<i32>, a_idx:i32, b_idx:i32, left:i32, right:i32) -> bool {
    println!("a_idx={}, b_idx={}, left={}, right={}", a_idx, b_idx, left, right);
    if a_idx == -1 || b_idx == -1 {
        println!("must be -1 both - {},{}", a_idx, b_idx);
        return a_idx == b_idx;
    }
    if a[a_idx as usize] != b[b_idx as usize] {
        return false;
    }

    let v = a[a_idx as usize];
    let smaller_a = find_smaller(a, a_idx as usize, left);
    let smaller_b = find_smaller(b, b_idx as usize, left);

    let bigger_a = find_bigger(a, a_idx as usize, right);
    let bigger_b = find_bigger(b, b_idx as usize, right);

    let l = chk(a, b, smaller_a, smaller_b, left, v);
    let r = chk(a, b, bigger_a, bigger_b, v, right);

    println!("l{} vs r{}", l, r);
    l && r
}

fn find_smaller(v:&Vec<i32>, base_idx:usize, left:i32) -> i32 {
    let base_v = v[base_idx];
    let mut idx = base_idx + 1;
    while idx < v.len() {
        if v[idx] < base_v && v[idx] >= left {
            return idx as i32;
        }
        idx += 1;
    }
    -1
}

fn find_bigger(v:&Vec<i32>, base_idx:usize, right:i32) -> i32 {
    let base_v = v[base_idx];
    let mut idx = base_idx + 1;
    while idx < v.len() {
        if v[idx] >= base_v && v[idx] < right {
            return idx as i32;
        }
        idx += 1;
    }
    -1
}

pub fn run() {
    let a = vec![10, 15, 8, 12, 94, 81, 5, 2, 11];
    let b = vec![10, 8, 5, 15, 2, 12, 11, 94, 81];
    println!("a={:?},b={:?}, is same BST? {}", a, b, exec(&a, &b));
}
