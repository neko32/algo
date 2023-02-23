
pub fn exec(v:&mut Vec<i32>) {
    let m_idx = (v.len() - 1) / 2;
    println!("orig - {:?}", v);
    for i in (0..=m_idx).rev() {
        println!("shift down with {i}:{}", v.len() - 1);
        shiftdown(v, i, v.len() - 1);
    }
    println!("heap built - {:?}", v);
}

fn shiftdown(v:&mut Vec<i32>, idx:usize, end_idx:usize) {
    let mut cld_idx_one = idx * 2 + 1;
    let mut idx = idx;
    while cld_idx_one <= end_idx {
        let cld_idx_two = if idx * 2 + 2 <= end_idx { idx * 2 + 2 } else { usize::MAX };
        let cld_to_swap = if cld_idx_two != usize::MAX && v[cld_idx_two] < v[cld_idx_one] {
            cld_idx_two
        } else {
            cld_idx_one
        };
        if v[idx] > v[cld_to_swap] {
            v.swap(idx, cld_to_swap);
            idx = cld_to_swap;
            cld_idx_one = idx * 2 + 1;
        } else {
            break;
        }
    }
}

pub fn run() {
    let mut v = vec![
        15, 12, 3, 24, 31,
        29, 21, 18, 25, 32,
    ];
    exec(&mut v);
}
