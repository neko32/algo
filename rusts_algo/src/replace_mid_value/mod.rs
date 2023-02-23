
pub fn exec(a:&[i32]) -> Vec<i32> {
    if a.len() == 1 {
        let mut v:Vec<i32> = Vec::new();
        a.iter().for_each(|x|v.push(*x));
        return v;
    }
    let mut buf:Vec<i32> = Vec::new();
    let len = a.len();
    let (st, ed, midv) = match len {
        n if n % 2 == 1 => {
            let mid_idx_st = len / 2;
            let mid_idx_ed = len / 2;
            let midv = a[mid_idx_st];
            (mid_idx_st, mid_idx_ed, midv)
        },
        n if n % 2 == 0 => {
            let mid_idx_st = len / 2 - 1;
            let mid_idx_ed = len / 2;
            let midv = a[mid_idx_st] + a[mid_idx_ed];
            (mid_idx_st, mid_idx_ed, midv)
        },
        _ => panic!("not possible"),
    };
    let s1 = &a[..st];
    let s2 = &a[ed + 1..];
    s1.into_iter().for_each(|x|buf.push(*x));
    buf.push(midv);
    s2.into_iter().for_each(|x|buf.push(*x));
    println!("input - {:?}, result - {:?}", a, buf);
    buf
}

pub fn run() {
    exec(&[7,2,2,5,10,7]);
    exec(&[-5, -5, 10]);
    exec(&[10]);
    exec(&[10, 20]);
}
